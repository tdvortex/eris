use std::sync::Arc;

use futures_util::future::{ready, Ready};
use thiserror::Error;
use tower::{Service, retry::{Policy, RetryLayer}, ServiceBuilder};
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


#[derive(Debug, Clone, Copy)]
struct RetryOnServerError;

impl<Res> Policy<Arc<DiscordClientAction>, Res, TwilightServiceError> for RetryOnServerError {
    type Future = Ready<Self>;

    fn retry(&self, _req: &Arc<DiscordClientAction>, result: Result<&Res, &TwilightServiceError>) -> Option<Self::Future> {
        let Err(e) = result else {
            return Some(ready(RetryOnServerError));
        };

        match e {
            TwilightServiceError::TwilightValidationError(_) => {
                // Client error, cannot retry
                None
            }
            TwilightServiceError::TwilightClientError(e) => {
                match e.kind() {
                    twilight_http::error::ErrorType::Response { body: _, error: _, status } => {
                        if status.is_server_error() {
                            // Something went wrong on Discord's side, retry
                            Some(ready(RetryOnServerError))
                        } else {
                            // Not a 5xx error, don't retry
                            None
                        }
                    }
                    twilight_http::error::ErrorType::ServiceUnavailable { response: _ } => {
                        // 503 error, retry
                        Some(ready(RetryOnServerError))
                    }
                    twilight_http::error::ErrorType::RequestTimedOut => {
                        // Network goblins, retry
                        Some(ready(RetryOnServerError))
                    }
                    _ =>  {
                        // Unknown error, not safe to retry
                        None
                    }
                }
            }
            TwilightServiceError::DeserializationBodyError(_) => {
                // Discord responded with a 2xx status code
                // but we couldn't deserialize the body
                // NOT safe to retry, request may not have been idempotent
                None
            }
        }
    }

    fn clone_request(&self, req: &Arc<DiscordClientAction>) -> Option<Arc<DiscordClientAction>> {
        Some(req.clone())
    }
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
    Error = TwilightServiceError,
> + Clone {
    let twilight_client = Arc::new(twilight_client);

    ServiceBuilder::new()
    .map_request(|action| Arc::new(action))
    .layer(RetryLayer::new(RetryOnServerError))
    .service_fn(move |request: Arc<DiscordClientAction>| {
        let twilight_client = twilight_client.clone();
        async move {
            match request.as_ref() {
                DiscordClientAction::CreateMessage(req) => create_message(&twilight_client, &req)
                    .await
                    .map(|message| Some(DiscordClientActionResponse::MessageCreated(message))),
                DiscordClientAction::CreateReply(req) => create_reply(&twilight_client, &req)
                    .await
                    .map(|message| Some(DiscordClientActionResponse::MessageCreated(message))),
                DiscordClientAction::DeleteMessage(req) => delete_message(&twilight_client, &req)
                    .await
                    .map(|_| Option::None),
                DiscordClientAction::UpdateInteractionResponse(req) => {
                    update_interaction_response(&twilight_client, application_id, &req)
                        .await
                        .map(|_| Option::None)
                }
                DiscordClientAction::UpdateMessage(req) => update_message(&twilight_client, &req)
                    .await
                    .map(|_| Option::None),
            }
        }
    })
}
