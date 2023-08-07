use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

use http::{Request, StatusCode};
use serde_json::Value as JsonValue;
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
        },
    },
    payloads::DiscordServerAction,
};

use super::interaction_response::{interaction_response_service, InteractionResponseError};

/// An error which might occur with the Discord endpoint.
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
    DeserializationError(serde_json::Error),
    /// Error queueing a received Interaction
    #[error("Error queueing a received Interaction: {0}")]
    QueueServiceError(Q::Error),
    /// Error serializing response
    #[error("Error trying to serialize a response: {0}")]
    SerializationError(serde_json::Error),
}

impl<B, Q> From<Infallible> for DiscordEndpointError<B, Q>
where
    B: http_body::Body,
    B::Error: Debug + Display,
    Q: Service<DiscordServerAction, Response = ()>,
    Q::Error: Debug + Display,
{
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
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
        .service(interaction_response_service(server_action_queue_service))
        .map_err(|e| -> DiscordEndpointError<B, Q> {
            match e {
                BodyToBytesServiceError::ToBytesError(e) => DiscordEndpointError::ToBytesError(e),
                BodyToBytesServiceError::InnerError(e) => match e {
                    DiscordSignatureVerificationLayerError::DiscordSignatureVerificationFailure(
                        e,
                    ) => DiscordEndpointError::DiscordSignatureVerificationError(e),
                    DiscordSignatureVerificationLayerError::InnerError(e) => match e {
                        JsonDeserializationServiceError::JsonDeserialization(e) => {
                            DiscordEndpointError::DeserializationError(e)
                        }
                        JsonDeserializationServiceError::InnerError(e) => match e {
                            InteractionResponseError::SerializationError(e) => {
                                DiscordEndpointError::SerializationError(e)
                            }
                            InteractionResponseError::QueueServiceError(e) => {
                                DiscordEndpointError::QueueServiceError(e)
                            }
                        },
                    },
                },
            }
        })
}
