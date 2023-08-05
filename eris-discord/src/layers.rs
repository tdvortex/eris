/// Authentication layer to verify Discord's signature on incoming Interactions
pub mod verify_signature;
pub use verify_signature::{verify_discord_signature_hyper, verify_discord_signature_lambda};

/// Service to queue posts to Discord, both Interaction responses and incoming messages
pub mod post_to_discord;