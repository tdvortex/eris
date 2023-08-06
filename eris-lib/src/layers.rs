/// A compatability [`tower::Layer`] which adapts an [`axum`]-compatible [`tower::Service`]
/// to a [`lambda_http`]-compatible one.
pub mod lambda_http_compatibility;

/// A [`tower::Layer`] which converts a [tower::Service] that takes a Request
/// and returns a Response that is Default, into one which takes an 
/// Option<Request> and returns the same Response type, using the default if
/// the input is None.
pub mod default_if_none;

/// A [`tower::Layer`] which converts a [tower::Service] that takes a [http::Request]
/// with a payload of [hyper::Body] into a [http::Request] with a payload of [hyper::body::Bytes].
pub mod body_to_bytes;

/// A [`tower::Layer`] which converts a [tower::Service] that takes a [http::Request]
/// with a payload of some T that is DeserializeOwned into a [http::Request] 
/// with a payload of [hyper::body::Bytes].
pub mod deserialize_json;

/// A [`tower::Layer`] which provides a queue service to another service.
pub mod queue_provider;

/// Authentication [`tower::Layer`] to verify Discord's [`ed25519_dalek::Signature`] on incoming
/// [`twilight_model::application::interaction::Interaction`]s before passing the [`http::Request`]
/// on to additional [`tower::Service`]s.
pub mod verify_signature;
pub use verify_signature::{verify_discord_signature_hyper, verify_discord_signature_lambda};

/// Authentication [`tower::Layer`] to verify Discord's [`ed25519_dalek::Signature`] on incoming
/// [`twilight_model::application::interaction::Interaction`]s before passing the [`http::Request`]
/// on to additional [`tower::Service`]s.
pub mod verify_signature_real;

/// A [`tower::Layer`] which provides [TwilightClientState] to a [tower::Service].
pub mod twilight_client_provider;
