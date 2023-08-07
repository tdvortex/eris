use std::fmt::{Debug, Display};
use thiserror::Error;
use tower::{Service, ServiceExt};
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::payloads::{DiscordClientAction, DiscordServerAction};

use super::twilight_service::{twilight_service, TwilightServiceError, TwilightValidationError};

/// An error that might be produced during the processing of a [DiscordClientAction].
#[derive(Debug, Error)]
pub enum DiscordClientActionServiceError<Q>
where
    Q: Service<DiscordServerAction>,
    Q::Error: Debug + Display,
{
    /// An error from the Twilight Discord client
    #[error("An error from the Twilight Discord client: {0}")]
    TwilightServiceError(#[from] TwilightServiceError),
    /// An error from the provided server action queue
    #[error("An error occurred in the queue service: {0}")]
    DiscordServerActionQueueError(Q::Error),
}

/// A service which receives a [DiscordClientAction] and sends it to Discord
/// through a rate-limited [twilight_http::Client]. If the response is
/// meaningful, ships it out through the provided queue service.
/// In case of an error, attempts to log the error using [tracing::error].
pub fn discord_client_action_service<Q>(
    twilight_client: twilight_http::Client,
    application_id: Id<ApplicationMarker>,
    mut server_action_queue_service: Q,
) -> impl Service<DiscordClientAction, Response = (), Error = DiscordClientActionServiceError<Q>> + Clone
where
    Q: Service<DiscordServerAction, Response = ()>,
    Q: Clone,
    Q::Error: Debug + Display,
{
    twilight_service(twilight_client, application_id)
        .then(|result_option_response| async move {
            match result_option_response {
                Ok(Some(discord_response)) => server_action_queue_service
                    .ready()
                    .await
                    .map_err(DiscordClientActionServiceError::DiscordServerActionQueueError)?
                    .call(DiscordServerAction::DiscordClientActionResponse(
                        discord_response,
                    ))
                    .await
                    .map_err(DiscordClientActionServiceError::DiscordServerActionQueueError),
                Ok(None) => Ok(()),
                Err(e) => Err(e.into()),
            }
        })
        .map_result(|result_response| match result_response {
            Ok(()) => Ok(()),
            Err(e) => {
                match e {
                    DiscordClientActionServiceError::TwilightServiceError(e) => match e {
                        TwilightServiceError::TwilightValidationError(e) => match e {
                            TwilightValidationError::MessageValidationError(e) => {
                                tracing::error!("twilight message validation error: {e}");
                            }
                            TwilightValidationError::ValidationError(e) => {
                                tracing::error!("twilight validation error: {e}");
                            }
                        },
                        TwilightServiceError::TwilightClientError(e) => {
                            tracing::error!("twilight client error: {e}");
                        }
                        TwilightServiceError::DeserializationBodyError(e) => {
                            tracing::error!("error deserializing Discord response: {e}");
                        }
                    },
                    DiscordClientActionServiceError::DiscordServerActionQueueError(e) => {
                        tracing::error!("server action queue failed: {e}");
                    }
                };
                Ok(())
            }
        })
}
