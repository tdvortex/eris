use std::sync::Arc;

use thiserror::Error;
use tower::{Service, ServiceBuilder};
use twilight_http::request::AuditLogReason;
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::{
    layers::twilight_client_provider_layer::{TwilightClientProviderLayer, TwilightClientState},
    payloads::{DiscordClientAction, DiscordClientActionResponse, MessagePayload},
};

/// Wrapper around two very similar validation errors from [twilight_validate].
#[derive(Debug, Error)]
pub enum TwilightValidationError {
    /// [twilight_validate::message::MessageValidationError]
    #[error("{0}")]
    MessageValidationError(#[from] twilight_validate::message::MessageValidationError),
    /// [twilight_validate::request::ValidationError]
    #[error("{0}")]
    ValidationError(#[from] twilight_validate::request::ValidationError),
}

/// Errors that can occur when constructing, sending, or receiving a response
/// to a Discord API call made via [twilight_http::Client].
#[derive(Debug, Error)]
pub enum TwilightServiceError {
    /// An error occurred during message validation, HTTP request not sent
    #[error("Message validation error: {0}")]
    TwilightValidationError(#[from] TwilightValidationError),
    /// The HTTP request was attempted but did not succeed, either due to a
    /// network failure, or because Discord returned an error status code
    #[error("Twilight client error: {0}")]
    TwilightClientError(#[from] twilight_http::error::Error),
    /// The HTTP request succeeded and a response was provided, but an error
    /// occurred while trying to deserialize the response from Discord.
    #[error("Error deserializing body from Discord response: {0}")]
    DeserializationBodyError(#[from] twilight_http::response::DeserializeBodyError),
}

async fn twilight_service_fn(
    (state, action): (Arc<TwilightClientState>, DiscordClientAction),
) -> Result<Option<DiscordClientActionResponse>, TwilightServiceError> {
    let twilight_client = &state.twilight_client;
    let application_id = state.application_id;
    match action {
        DiscordClientAction::CreateMessage(create_message) => {
            let request = twilight_client.create_message(create_message.channel_id);
            let response = match &create_message.message {
                MessagePayload::Text(text) => {
                    request
                        .content(text)
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::Embed(embed) => {
                    request
                        .embeds(std::slice::from_ref(embed))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::TextAndEmbed { text, embed } => {
                    request
                        .content(text)
                        .map_err(TwilightValidationError::from)?
                        .embeds(std::slice::from_ref(embed))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
            };

            let message = response.model().await?;
            Ok(Some(DiscordClientActionResponse::MessageCreated(message)))
        }
        DiscordClientAction::CreateReply(create_reply) => {
            let request = twilight_client
                .create_message(create_reply.message_location.channel_id)
                .reply(create_reply.message_location.message_id)
                .fail_if_not_exists(false);
            let response = match &create_reply.message {
                MessagePayload::Text(text) => {
                    request
                        .content(text)
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::Embed(embed) => {
                    request
                        .embeds(std::slice::from_ref(embed))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::TextAndEmbed { text, embed } => {
                    request
                        .content(text)
                        .map_err(TwilightValidationError::from)?
                        .embeds(std::slice::from_ref(embed))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
            };

            let message = response.model().await?;
            Ok(Some(DiscordClientActionResponse::MessageCreated(message)))
        }
        DiscordClientAction::DeleteMessage(delete_message) => {
            let mut request = twilight_client.delete_message(
                delete_message.message_location.channel_id,
                delete_message.message_location.message_id,
            );

            if let Some(reason) = &delete_message.reason {
                request = request
                    .reason(reason)
                    .map_err(TwilightValidationError::from)?;
            }

            request.await?;
            Ok(None)
        }
        DiscordClientAction::UpdateInteractionResponse(interaction_response) => {
            let interaction_client = twilight_client.interaction(application_id);

            let request =
                interaction_client.update_response(&interaction_response.interaction_token);

            match &interaction_response.message {
                MessagePayload::Text(text) => {
                    request
                        .content(Some(text))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::Embed(embed) => {
                    request
                        .embeds(Some(std::slice::from_ref(embed)))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::TextAndEmbed { text, embed } => {
                    request
                        .content(Some(text))
                        .map_err(TwilightValidationError::from)?
                        .embeds(Some(std::slice::from_ref(embed)))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
            };
            Ok(None)
        }
        DiscordClientAction::UpdateMessage(update_message) => {
            let request = twilight_client.update_message(
                update_message.message_location.channel_id,
                update_message.message_location.message_id,
            );

            match &update_message.message {
                MessagePayload::Text(text) => {
                    request
                        .content(Some(text))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::Embed(embed) => {
                    request
                        .embeds(Some(std::slice::from_ref(embed)))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
                MessagePayload::TextAndEmbed { text, embed } => {
                    request
                        .content(Some(text))
                        .map_err(TwilightValidationError::from)?
                        .embeds(Some(std::slice::from_ref(embed)))
                        .map_err(TwilightValidationError::from)?
                        .await?
                }
            };
            Ok(None)
        }
    }
}

/// Returns a [tower::Service] which processes [DiscordClientAction]s and
/// sometimes returns a [DiscordClientActionResponse] for additional
/// processing.
pub fn twilight_service(
    twilight_client: twilight_http::Client,
    application_id: Id<ApplicationMarker>,
) -> impl Service<
    DiscordClientAction,
    Response = Option<DiscordClientActionResponse>,
    Error = TwilightServiceError,
> {
    ServiceBuilder::new()
        .layer(TwilightClientProviderLayer::new(
            twilight_client,
            application_id,
        ))
        .service_fn(twilight_service_fn)
}
