/// A service which makes calls to the Discord API.
pub mod discord_client;
/// A service which receives POST requests made by Discord,
/// fowards them onto a handler service, and immediately responds with
/// DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE to prevent timeouts.
pub mod discord_endpoint;

/// A service which asynchronoulsy handles POST requests made by Discord, as 
/// well as meaningful responses to client requests.
pub mod discord_handler;

/// A service which reads and writes to a Postgres database.
pub mod postgres_client;

/// A service which makes GET and POST requests to external ActivityPub APIs.
pub mod activitypub_client;