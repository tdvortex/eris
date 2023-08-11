use std::fmt::{Debug, Display};

use axum::response::IntoResponse;
use http::StatusCode;
use serde_json::Value as JsonValue;
use thiserror::Error;
use tower::{Service, ServiceBuilder, ServiceExt};
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

use crate::{
    layers::provide_cloned_state::ClonedStateProviderLayer, payloads::DiscordServerAction,
};

/// The response to a PING request.
pub const PONG: InteractionResponse = InteractionResponse {
    kind: InteractionResponseType::Pong,
    data: None,
};

/// A response telling Discord that we will respond to this interaction
/// later.
pub const DEFER: InteractionResponse = InteractionResponse {
    kind: InteractionResponseType::DeferredChannelMessageWithSource,
    data: None,
};

/// An error attempting to respond to an Interaction.
#[derive(Debug, Error)]
pub enum InteractionResponseError<Q: Debug + Display> {
    /// Could not serialize the JSON response.
    #[error("Error while serializing to JSON: {0}")]
    SerializationError(#[from] serde_json::Error),
    /// Could not queue
    #[error("Error from the queue service: {0}")]
    QueueServiceError(Q),
}

async fn respond_to_interaction<Q>(
    (mut queue_service, interaction): (Q, Interaction),
) -> Result<(StatusCode, JsonValue), InteractionResponseError<Q::Error>>
where
    Q: Service<DiscordServerAction, Response = ()>,
    Q::Error: Debug + Display,
{
    // If the interaction is just a PING we do not need to queue it
    if interaction.kind == InteractionType::Ping {
        return Ok((StatusCode::OK, serde_json::to_value(PONG)?));
    }

    queue_service
        .ready()
        .await
        .map_err(InteractionResponseError::QueueServiceError)?
        .call(DiscordServerAction::from(interaction))
        .await
        .map_err(InteractionResponseError::QueueServiceError)?;

    Ok((StatusCode::OK, serde_json::to_value(DEFER)?))
}

/// Returns a service which takes an incoming Interaction, queues it, and
/// responds as quickly as possible with 200 OK and DEFERRED_CHANNEL_MESSAGE.
pub fn respond_to_interaction_layer_fn<Q>(
    queue_service: Q,
) -> impl Service<
    Interaction,
    Response = impl IntoResponse,
    Error = InteractionResponseError<Q::Error>,
> + Clone
where
    Q: Service<DiscordServerAction, Response = ()>,
    Q: Clone,
    Q::Error: Debug + Display,
{
    ServiceBuilder::new()
        .layer(ClonedStateProviderLayer::new(queue_service))
        .service_fn(respond_to_interaction)
        .map_response(|(status_code, json)| (status_code, axum::Json(json)))
}
