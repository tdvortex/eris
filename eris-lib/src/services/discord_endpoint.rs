use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

use http::{Request, StatusCode};
use serde_json::Value as JsonValue;
use thiserror::Error;
use tower::{Service, ServiceBuilder};
use twilight_model::application::interaction::Interaction;

use crate::{
    layers::{
        body_to_bytes::body_to_bytes_layer_fn,
        deserialize_json::deserialize_json_layer_fn,
        verify_signature::verify_discord_signature_layer,
    },
    payloads::DiscordServerAction,
};

use super::interaction_response::interaction_response_service;

#[derive(Debug, Error)]
pub enum DiscordEndpointError<B, Q>
where
    B: http_body::Body,
    B::Error: Debug + Display,
    Q: Service<DiscordServerAction, Response = ()>,
    Q::Error: Debug + Display,
{
    #[error("Error deserializing Interaction from body")]
    DeserializationError(serde_json::Error),
    #[error("Error queueing a received Interaction: {0}")]
    QueueServiceError(Q::Error),
    #[error("Error trying to serialize a response: {0}")]
    ResponseSerializationError(serde_json::Error),
    #[error("Error extracting Bytes from request body: {0}")]
    ToBytesError(B::Error),
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

fn discord_endpoint_service<B, Q>(
    public_key: ed25519_dalek::PublicKey,
    server_action_queue_service: Q,
) -> impl Service<Request<B>, Response = (StatusCode, JsonValue)>
where
    B: http_body::Body,
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
        .service(interaction_response_service(queue_service))
}
