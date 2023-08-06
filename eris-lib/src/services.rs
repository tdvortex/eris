/// A service which receives POST requests made by Discord,
/// fowards them onto a handler service, and immediately responds with
/// DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE to prevent timeouts.
pub mod discord_endpoint;

/// A service which tak
pub mod twilight_service;