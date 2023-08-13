use std::sync::Arc;

use thiserror::Error;
use tower::{service_fn, Service};
use twilight_http::request::AuditLogReason;
use twilight_model::{
    channel::Message,
    id::{marker::ApplicationMarker, Id},
};

use crate::payloads::{
    CreateMessage, CreateReply, DeleteMessage, DiscordClientAction, DiscordClientActionResponse,
    MessagePayload, UpdateInteractionResponse, UpdateMessage,
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

async fn create_message(
    twilight_client: &twilight_http::Client,
    create_message: &CreateMessage,
) -> Result<Message, TwilightServiceError> {
    let request = twilight_client.create_message(create_message.channel_id);
    let response = match &create_message.message {
        MessagePayload::Text(text) => {
            request
                .content(text)
                .map_err(TwilightValidationError::MessageValidationError)?
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
    Ok(message)
}

async fn create_reply(
    twilight_client: &twilight_http::Client,
    create_reply: &CreateReply,
) -> Result<Message, TwilightServiceError> {
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
    Ok(message)
}

async fn delete_message(
    twilight_client: &twilight_http::Client,
    delete_message: &DeleteMessage,
) -> Result<(), TwilightServiceError> {
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
    Ok(())
}

async fn update_interaction_response(
    twilight_client: &twilight_http::Client,
    application_id: Id<ApplicationMarker>,
    update_interaction_response: &UpdateInteractionResponse,
) -> Result<(), TwilightServiceError> {
    let interaction_client = twilight_client.interaction(application_id);

    let request =
        interaction_client.update_response(&update_interaction_response.interaction_token);

    match &update_interaction_response.message {
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
    Ok(())
}

async fn update_message(
    twilight_client: &twilight_http::Client,
    update_message: &UpdateMessage,
) -> Result<(), TwilightServiceError> {
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
    Ok(())
}



/// Returns a [tower::Service] which processes [DiscordClientAction]s and
/// sometimes returns a [DiscordClientActionResponse] for additional
/// processing. If it errors, returns the offending action as well as the error.
pub fn twilight_service(
    twilight_client: twilight_http::Client,
    application_id: Id<ApplicationMarker>,
) -> impl Service<
    DiscordClientAction,
    Response = Option<DiscordClientActionResponse>,
    Error = (DiscordClientAction, TwilightServiceError),
> + Clone {
    let twilight_client = Arc::new(twilight_client);

    service_fn(move |request: DiscordClientAction| {
        let twilight_client = twilight_client.clone();
        async move {
            match &request {
                DiscordClientAction::CreateMessage(req) => create_message(&twilight_client, &req)
                    .await
                    .map(|message| Some(DiscordClientActionResponse::MessageCreated(message)))
                    .map_err(|error| (request, error)),
                DiscordClientAction::CreateReply(req) => create_reply(&twilight_client, &req)
                    .await
                    .map(|message| Some(DiscordClientActionResponse::MessageCreated(message)))
                    .map_err(|error| (request, error)),
                DiscordClientAction::DeleteMessage(req) => delete_message(&twilight_client, &req)
                    .await
                    .map(|_| Option::None)
                    .map_err(|error| (request, error)),
                DiscordClientAction::UpdateInteractionResponse(req) => {
                    update_interaction_response(&twilight_client, application_id, &req)
                        .await
                        .map(|_| Option::None)
                        .map_err(|error| {
                            (request, error)
                        })
                }
                DiscordClientAction::UpdateMessage(req) => update_message(&twilight_client, &req)
                    .await
                    .map(|_| Option::None)
                    .map_err(|error| (request, error)),
            }
        }
    })
}
