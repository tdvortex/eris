use std::fmt::{Debug, Display};

use axum::{middleware::Next, response::Response, Json};
use http::{Request, StatusCode};
use hyper::Body as HyperBody;
use serde_json::Value as JsonValue;
use thiserror::Error;
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};

use crate::{
    layers::queue_provider::{QueueProviderLayer, QueueProviderLayerError},
    payloads::DiscordServerAction,
};

/// The response to a PING request.
const PONG: InteractionResponse = InteractionResponse {
    kind: InteractionResponseType::Pong,
    data: None,
};

const DEFER: InteractionResponse = InteractionResponse {
    kind: InteractionResponseType::DeferredChannelMessageWithSource,
    data: None,
};

#[derive(Debug, Error)]
pub enum InteractionResponseError<Q: Debug + Display> {
    #[error("Error while serializing to JSON: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error from the queue service: {0}")]
    QueueServiceError(Q),
}

async fn response_to_interaction<Q>(
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
        .call(DiscordServerAction::from(interaction))
        .await
        .map_err(|e| InteractionResponseError::QueueServiceError(e))?;

    Ok((StatusCode::OK, serde_json::to_value(DEFER)?))
}

fn interaction_response_service<Q>(
    queue_service: Q,
) -> impl Service<
    Interaction,
    Response = (StatusCode, JsonValue),
    Error = InteractionResponseError<Q::Error>,
>
where
    Q: Service<DiscordServerAction, Response = ()>,
    Q: Clone,
    Q::Error: Debug + Display,
{
    ServiceBuilder::new()
        .layer(QueueProviderLayer::new(queue_service))
        .service_fn(response_to_interaction)
        .map_err(|e| match e {
            QueueProviderLayerError::QueueError(e)
            | QueueProviderLayerError::InnerError(InteractionResponseError::QueueServiceError(e)) => InteractionResponseError::QueueServiceError(e),
            QueueProviderLayerError::InnerError(e) => e,
        })
}
