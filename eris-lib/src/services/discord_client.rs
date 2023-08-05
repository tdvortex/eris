/// The defined actions that can be made to the Discord API.
pub mod client_actions;
use std::{future::Future, pin::Pin, sync::Arc};

pub use client_actions::DiscordClientAction;
use thiserror::Error;
use tower::{Layer, Service, service_fn};

/// The meaningful non-error responses we might get from the Discord API.
pub mod client_action_response;

mod error;
pub use error::Error as DiscordClientServiceError;
use twilight_http::request::AuditLogReason;
use twilight_model::id::{marker::ApplicationMarker, Id};

use self::client_action_response::ClientActionResponse;

/// The trait bound for a DiscordClientService.
pub trait DiscordClientService:
    Service<DiscordClientAction, Response = (), Error = DiscordClientServiceError>
{
}

#[derive(Debug, Error)]
enum TwilightValidationError {
    #[error("{0}")]
    MessageValidationError(#[from] twilight_validate::message::MessageValidationError),
    #[error("{0}")]
    ValidationError(#[from] twilight_validate::request::ValidationError),
}

#[derive(Debug, Error)]
enum TwilightServiceError {
    #[error("Message validation error: {0}")]
    TwilightValidationError(#[from] TwilightValidationError),
    #[error("Twilight client error: {0}")]
    TwilightClientError(#[from] twilight_http::error::Error),
    #[error("Error deserializing body from Discord: {0}")]
    DeserializationBodyError(#[from] twilight_http::response::DeserializeBodyError),
}

#[derive(Debug)]
struct TwilightServiceState {
    twilight_client: twilight_http::Client,
    application_id: Id<ApplicationMarker>,
}

async fn do_action(
    state_request: (Arc<TwilightServiceState>, DiscordClientAction),
) -> Result<Option<ClientActionResponse>, TwilightServiceError> {
    let (state, action) = state_request;

    state.do_action(action).await
}

impl TwilightServiceState {
    async fn do_action(
        &self,
        action: DiscordClientAction,
    ) -> Result<Option<ClientActionResponse>, TwilightServiceError> {
        match action {
            DiscordClientAction::CreateMessage(create_message) => {
                let request = self
                    .twilight_client
                    .create_message(create_message.channel_id);
                let response = match &create_message.message {
                    client_actions::MessagePayload::Text(text) => {
                        request
                            .content(text)
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::Embed(embed) => {
                        request
                            .embeds(std::slice::from_ref(embed))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::TextAndEmbed { text, embed } => {
                        request
                            .content(text)
                            .map_err(TwilightValidationError::from)?
                            .embeds(std::slice::from_ref(embed))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                };

                let message = response.model().await?;
                Ok(Some(ClientActionResponse::MessageCreated(message)))
            }
            DiscordClientAction::CreateReply(create_reply) => {
                let request = self
                    .twilight_client
                    .create_message(create_reply.message_location.channel_id)
                    .reply(create_reply.message_location.message_id)
                    .fail_if_not_exists(false);
                let response = match &create_reply.message {
                    client_actions::MessagePayload::Text(text) => {
                        request
                            .content(text)
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::Embed(embed) => {
                        request
                            .embeds(std::slice::from_ref(embed))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::TextAndEmbed { text, embed } => {
                        request
                            .content(text)
                            .map_err(TwilightValidationError::from)?
                            .embeds(std::slice::from_ref(embed))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                };

                let message = response.model().await?;
                Ok(Some(ClientActionResponse::MessageCreated(message)))
            }
            DiscordClientAction::DeleteMessage(delete_message) => {
                let mut request = self.twilight_client.delete_message(
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
                let interaction_client = self.twilight_client.interaction(self.application_id);

                let request =
                    interaction_client.update_response(&interaction_response.interaction_token);

                match &interaction_response.message {
                    client_actions::MessagePayload::Text(text) => {
                        request
                            .content(Some(text))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::Embed(embed) => {
                        request
                            .embeds(Some(std::slice::from_ref(embed)))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::TextAndEmbed { text, embed } => {
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
                let request = self.twilight_client.update_message(
                    update_message.message_location.channel_id,
                    update_message.message_location.message_id,
                );

                match &update_message.message {
                    client_actions::MessagePayload::Text(text) => {
                        request
                            .content(Some(text))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::Embed(embed) => {
                        request
                            .embeds(Some(std::slice::from_ref(embed)))
                            .map_err(TwilightValidationError::from)?
                            .await?
                    }
                    client_actions::MessagePayload::TextAndEmbed { text, embed } => {
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
}

struct TwilightServiceStateLayer {
    state: Arc<TwilightServiceState>,
}

impl TwilightServiceStateLayer {
    pub fn new(
        twilight_client: twilight_http::Client,
        application_id: Id<ApplicationMarker>,
    ) -> Self {
        Self {
            state: Arc::new(TwilightServiceState {
                twilight_client,
                application_id,
            }),
        }
    }
}

impl<S> Layer<S> for TwilightServiceStateLayer {
    type Service = TwilightService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TwilightService {
            state: self.state.clone(),
            inner,
        }
    }
}

struct TwilightService<S> {
    state: Arc<TwilightServiceState>,
    inner: S,
}

impl<S, Request> Service<Request> for TwilightService<S>
where
    S: Service<(Arc<TwilightServiceState>, Request)>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call((self.state.clone(), req))
    }
}

fn create_twilight_service(twilight_client: twilight_http::Client, application_id: Id<ApplicationMarker>) -> impl Service<DiscordClientAction, Response = Option<ClientActionResponse>, Error = TwilightServiceError> {
    let layer = TwilightServiceStateLayer::new(twilight_client, application_id);
    let service_fn = service_fn(do_action);

    layer.layer(service_fn)
}
