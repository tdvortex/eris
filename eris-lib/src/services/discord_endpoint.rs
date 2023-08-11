use std::fmt::{Debug, Display};

use axum::response::IntoResponse;
use http::{Request, StatusCode};
use thiserror::Error;
use tower::{Service, ServiceBuilder, ServiceExt};
use twilight_model::application::interaction::Interaction;

use crate::{
    layers::{
        body_to_bytes::{body_to_bytes_layer_fn, BodyToBytesServiceError},
        deserialize_json::{deserialize_json_layer_fn, JsonDeserializationServiceError},
        respond_to_interaction::{respond_to_interaction_layer_fn, InteractionResponseError},
        verify_signature::{
            verify_discord_signature_layer, DiscordSignatureVerificationFailure,
            DiscordSignatureVerificationLayerError,
        },
    },
    payloads::DiscordServerAction,
};

/// An error which might occur with the Discord endpoint.
/// Triggers either a 4xx response with a JSON payload,
/// or a 5xx response combined with logging the error using tracing
#[derive(Debug, Error)]
pub enum DiscordEndpointError<B, Q>
where
    B: http_body::Body,
    B::Error: Debug + Display,
    Q: Service<DiscordServerAction, Response = ()>,
    Q::Error: Debug + Display,
{
    /// Error extracting Bytes from request body
    #[error("Error extracting Bytes from request body: {0}")]
    ToBytesError(B::Error),
    /// Error validating Discord signature
    #[error("Error validating Discord signature")]
    DiscordSignatureVerificationError(#[from] DiscordSignatureVerificationFailure),
    /// Error deserializing Interaction from body
    #[error("Error deserializing Interaction from request body: {0}")]
    JsonDeserializationError(serde_json::Error),
    /// Error queueing a received Interaction
    #[error("Error queueing a received Interaction: {0}")]
    QueueServiceError(Q::Error),
    /// Error serializing response
    #[error("Error trying to serialize a response: {0}")]
    SerializationError(serde_json::Error),
}

/// A service which receives an HTTP request and returns a reply in the form of
/// a (StatusCode, JsonValue) pair.
pub fn discord_endpoint_service<B, Q>(
    public_key: ed25519_dalek::PublicKey,
    server_action_queue_service: Q,
) -> impl Service<Request<B>, Response = axum::response::Response, Error = DiscordEndpointError<B, Q>>
       + Clone
where
    B: http_body::Body + Clone,
    B::Error: Debug + Display,
    Q: Service<DiscordServerAction, Response = ()>,
    Q: Clone,
    Q::Error: Debug + Display,
{
    ServiceBuilder::new()
        .layer_fn(body_to_bytes_layer_fn)
        .layer(verify_discord_signature_layer(public_key))
        .layer_fn(deserialize_json_layer_fn)
        .map_request(|request: Request<Interaction>| request.into_body())
        .layer_fn(respond_to_interaction_layer_fn)
        .service(server_action_queue_service)
        .map_response(axum::response::IntoResponse::into_response)
        .map_err(|e| {
            let e = match e {
                BodyToBytesServiceError::ToBytesError(e) => {
                    return DiscordEndpointError::ToBytesError(e);
                }
                BodyToBytesServiceError::InnerError(e) => e,
            };

            let e = match e {
                DiscordSignatureVerificationLayerError::DiscordSignatureVerificationFailure(e) => {
                    return DiscordEndpointError::DiscordSignatureVerificationError(e);
                }
                DiscordSignatureVerificationLayerError::InnerError(e) => e,
            };

            let e = match e {
                JsonDeserializationServiceError::JsonDeserialization(e) => {
                    return DiscordEndpointError::JsonDeserializationError(e);
                }
                JsonDeserializationServiceError::InnerError(e) => e,
            };

            match e {
                InteractionResponseError::SerializationError(e) => {
                    DiscordEndpointError::SerializationError(e)
                }
                InteractionResponseError::QueueServiceError(e) => {
                    DiscordEndpointError::QueueServiceError(e)
                }
            }
        })
        .map_result(|result_response| {
            let Err(e) = result_response else {
                return result_response;
            };

            match e {
                DiscordEndpointError::ToBytesError(_) => {
                    Ok(StatusCode::BAD_REQUEST.into_response())
                }
                DiscordEndpointError::DiscordSignatureVerificationError(e) => match e {
                    DiscordSignatureVerificationFailure::MissingPublicKey => {
                        tracing::error!("Discord endpoint service does not have Discord public key middlware");
                        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                    }
                    DiscordSignatureVerificationFailure::MissingHeader(_)
                    | DiscordSignatureVerificationFailure::BadFormat(_)
                    | DiscordSignatureVerificationFailure::InvalidSignature => {
                        Ok(StatusCode::UNAUTHORIZED.into_response())
                    }
                }
                DiscordEndpointError::JsonDeserializationError(_) => {
                    Ok(StatusCode::BAD_REQUEST.into_response())
                }
                // Pass through queue service errors; handling defined by input service
                DiscordEndpointError::QueueServiceError(_) => Err(e),
                DiscordEndpointError::SerializationError(e) => {
                    tracing::error!("Unable to serialize a response: {e}");
                    Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                }
            }
        })
}
