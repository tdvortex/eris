/// A service which receives POST requests made by Discord,
/// fowards them onto a handler service, and immediately responds with
/// DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE to prevent timeouts.
pub mod discord_endpoint;

/// A service which receives a [DiscordClientAction] as a request
/// and returns () as a response if successful.
pub mod discord_client_action_service;

/// A service which responds to an Interaction by queuing it and responding
/// with DEFERRED_CHANNEL_MESSAGE as quickly as possible.
pub mod interaction_response;

/// A[tower::Service] which processes [DiscordClientAction]s and
/// sometimes returns a [DiscordClientActionResponse] for additional
/// processing.
pub mod twilight_service;