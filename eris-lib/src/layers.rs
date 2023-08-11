/// A [`tower::Layer`] which converts a [tower::Service] that takes a [http::Request]
/// with a payload of [hyper::Body] into a [http::Request] with a payload of [hyper::body::Bytes].
pub mod body_to_bytes;
/// A [`tower::Layer`] which converts a [tower::Service] that takes a service 
/// taking a request of type (T, R) into one that only takes type R, by cloning
/// a shared state T and passing it as the first argument.
/// This is somewhat clunky; when possible [tower_http::add_extension::AddExtensionLayer]
pub mod provide_cloned_state;

/// A [`tower::Layer`] which converts a [tower::Service] that takes a [http::Request]
/// with a payload of some T that is DeserializeOwned into a [http::Request] 
/// with a payload of [hyper::body::Bytes].
pub mod deserialize_json;

/// A [`tower::Layer`] which constructs a [tower::Service] that responds to an
/// Interaction by queuing it and responding with DEFERRED_CHANNEL_MESSAGE as
/// quickly as possible. Must be provided with a service that takes a
/// [DiscordServerAction].
pub mod respond_to_interaction;

/// Authentication [`tower::Layer`] to verify Discord's [`ed25519_dalek::Signature`] on incoming
/// [`twilight_model::application::interaction::Interaction`]s before passing the [`http::Request`]
/// on to additional [`tower::Service`]s.
pub mod verify_signature;
