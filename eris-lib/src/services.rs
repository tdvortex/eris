/// A service which receives POST requests made by Discord,
/// fowards them onto a handler service, and immediately responds with
/// DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE to prevent timeouts.
pub mod discord_endpoint;
/// A service which receives [DiscordClientAction]s and sends them to Discord,
/// and queues any responses.
pub mod discord_client_action;



/// A[tower::Service] which processes [DiscordClientAction]s and
/// sometimes returns a [DiscordClientActionResponse] for additional
/// processing.
pub mod twilight_service;