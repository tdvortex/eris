use std::fmt::{Debug, Display};

use http::{Request, StatusCode};
use serde_json::{json, Value as JsonValue};
use thiserror::Error;
use tower::{Service, ServiceBuilder, ServiceExt};
use twilight_model::application::interaction::Interaction;

use crate::{
    layers::{
        body_to_bytes::{body_to_bytes_layer_fn, BodyToBytesServiceError},
        deserialize_json::{deserialize_json_layer_fn, JsonDeserializationServiceError},
        verify_signature::{
            verify_discord_signature_layer, DiscordSignatureVerificationFailure,
            DiscordSignatureVerificationLayerError,
        }, respond_to_interaction::{respond_to_interaction_layer_fn, InteractionResponseError},
    },
    payloads::DiscordServerAction,
};

/// An error while trying to parse or validate the request.
/// Triggers a 4xx response code and a JSON explanation.
#[derive(Debug, Error)]
pub enum DiscordEndpointClientError<B>
where
    B: http_body::Body,
    B::Error: Debug + Display,
{
    /// Error extracting Bytes from request body
    #[error("Error extracting Bytes from request body: {0}")]
    ToBytesError(B::Error),
    /// Error validating Discord signature
    #[error("Error validating Discord signature")]
    DiscordSignatureVerificationError(#[from] DiscordSignatureVerificationFailure),
    /// Error deserializing Interaction from body
    #[error("Error deserializing Interaction from request body: {0}")]
    DeserializationError(#[from] serde_json::Error),
}

/// An error trying to service a valid request.
/// Triggers a 5xx response code and logging the error.
#[derive(Debug, Error)]
pub enum DiscordEndpointServerError<Q>
where
    Q: Service<DiscordServerAction, Response = ()>,
    Q::Error: Debug + Display,
{
    /// Error queueing a received Interaction
    #[error("Error queueing a received Interaction: {0}")]
    QueueServiceError(Q::Error),
    /// Error serializing response
    #[error("Error trying to serialize a response: {0}")]
    SerializationError(#[from] serde_json::Error),
}

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
    /// An error while trying to parse or validate the request.
    /// Triggers a 4xx response.
    #[error("Error interpreting the request: {0}")]
    DiscordEndpointClientError(#[from] DiscordEndpointClientError<B>),
    /// An error trying to service a valid request.
    /// Triggers a 5xx request.
    #[error("Error handling a valid request: {0}")]
    DiscordEndpointServerError(#[from] DiscordEndpointServerError<Q>),
}

/// A service which receives an HTTP request and returns a reply in the form of
/// a (StatusCode, JsonValue) pair.
pub fn discord_endpoint_service<B, Q>(
    public_key: ed25519_dalek::PublicKey,
    server_action_queue_service: Q,
) -> impl Service<Request<B>, Response = (StatusCode, JsonValue), Error = DiscordEndpointError<B, Q>>
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
        .map_err(|e| -> DiscordEndpointError<B, Q> {
            match e {
                BodyToBytesServiceError::ToBytesError(e) => {
                    DiscordEndpointError::DiscordEndpointClientError(
                        DiscordEndpointClientError::ToBytesError(e),
                    )
                }
                BodyToBytesServiceError::InnerError(e) => match e {
                    DiscordSignatureVerificationLayerError::DiscordSignatureVerificationFailure(
                        e,
                    ) => DiscordEndpointError::DiscordEndpointClientError(
                        DiscordEndpointClientError::DiscordSignatureVerificationError(e),
                    ),
                    DiscordSignatureVerificationLayerError::InnerError(e) => match e {
                        JsonDeserializationServiceError::JsonDeserialization(e) => {
                            DiscordEndpointError::DiscordEndpointClientError(e.into())
                        }
                        JsonDeserializationServiceError::InnerError(e) => match e {
                            InteractionResponseError::SerializationError(e) => {
                                DiscordEndpointError::DiscordEndpointServerError(e.into())
                            }
                            InteractionResponseError::QueueServiceError(e) => {
                                DiscordEndpointError::DiscordEndpointServerError(
                                    DiscordEndpointServerError::QueueServiceError(e),
                                )
                            }
                        },
                    },
                },
            }
        })
        .map_result(|response_result| match response_result {
            Ok(response) => Ok(response),
            Err(DiscordEndpointError::DiscordEndpointClientError(e)) => match e {
                DiscordEndpointClientError::ToBytesError(_) => Ok((
                    StatusCode::BAD_REQUEST,
                    json!({"error": "invalid request body"}),
                )),
                DiscordEndpointClientError::DiscordSignatureVerificationError(e) => match e {
                    DiscordSignatureVerificationFailure::MissingHeader(header_name) => {
                        let error = format!("missing header: {header_name}");
                        Ok((
                            StatusCode::UNAUTHORIZED,
                            json!({
                                "error": error
                            }),
                        ))
                    }
                    DiscordSignatureVerificationFailure::BadFormat(error) => Ok((
                        StatusCode::UNAUTHORIZED,
                        json!({
                            "error": error
                        }),
                    )),
                    DiscordSignatureVerificationFailure::InvalidSignature => Ok((
                        StatusCode::UNAUTHORIZED,
                        json!({
                            "error": "invalid signature"
                        }),
                    )),
                },
                DiscordEndpointClientError::DeserializationError(_) => Ok((
                    StatusCode::BAD_REQUEST,
                    json!({"error": "could not interpret interaction"}),
                )),
            },
            Err(DiscordEndpointError::DiscordEndpointServerError(e)) => {
                match e {
                    DiscordEndpointServerError::QueueServiceError(e) => {
                        tracing::error!("server action queue failed: {e}");
                    }
                    DiscordEndpointServerError::SerializationError(e) => {
                        tracing::error!("serializing response failed: {e}");
                    }
                };

                // Don't tell Discord how we screwed up
                Ok((StatusCode::BAD_REQUEST, json!({"error": "internal server error"})))
            }
        })
}
