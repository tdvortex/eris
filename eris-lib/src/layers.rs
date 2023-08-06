/// A compatability [`tower::Layer`] which adapts an [`axum`]-compatible [`tower::Service`]
/// to a [`lambda_http`]-compatible one.
pub mod lambda_http_compatibility;

/// Authentication [`tower::Layer`] to verify Discord's [`ed25519_dalek::Signature`] on incoming
/// [`twilight_model::application::interaction::Interaction`]s before passing the [`http::Request`]
/// on to additional [`tower::Service`]s.
pub mod verify_signature;
pub use verify_signature::{verify_discord_signature_hyper, verify_discord_signature_lambda};


/// Authentication [`tower::Layer`] to verify Discord's [`ed25519_dalek::Signature`] on incoming
/// [`twilight_model::application::interaction::Interaction`]s before passing the [`http::Request`]
/// on to additional [`tower::Service`]s.
pub mod verify_signature_real;

