use thiserror::Error;

use crate::layers::verify_signature_real::DiscordVerificationServiceError;

/// An error that can be produced during the processing of a call to the
/// Discord endpoint.
#[derive(Debug, Error)]
pub enum DiscordEndpointError {
    /// An error while attempting to verify the signature
    #[error("An error occured while attempting to verify the signature: {0}")]
    DiscordVerificationService(#[from] DiscordVerificationServiceError),
}