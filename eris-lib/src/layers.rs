/// Authentication layer to verify Discord's signature on incoming Interactions
pub mod verify_signature;
pub use verify_signature::{verify_discord_signature_hyper, verify_discord_signature_lambda};