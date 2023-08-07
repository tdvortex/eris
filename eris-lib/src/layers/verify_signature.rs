use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

use axum::RequestPartsExt;
use ed25519_dalek::Verifier;
use http::{HeaderMap, Request};
use hyper::body::Bytes;
use thiserror::Error;
use tower::{layer::layer_fn, service_fn, Layer, Service, ServiceBuilder, ServiceExt};

use super::provide_cloned_state::ClonedStateProviderLayer;

/// A reason for the signature validation failure
#[derive(Debug, Error)]
pub enum DiscordSignatureVerificationFailure {
    /// Missing a required header
    #[error("Missing header: {0}")]
    MissingHeader(&'static str),
    /// Header is present but improperly formatted
    #[error("Bad formatting: {0}")]
    BadFormat(&'static str),
    /// Signature is present, but cannot be used to validate request
    #[error("InvalidSignature")]
    InvalidSignature,
}

impl From<Infallible> for DiscordSignatureVerificationFailure {
    fn from(_infallible: Infallible) -> Self {
        unreachable!()
    }
}

/// An error from the DiscordSignatureVerificationLayer, possibly
/// passed up from an inner service
#[derive(Debug, Error)]
pub enum DiscordSignatureVerificationLayerError<E: Debug + Display> {
    /// Signature verification failed
    #[error("{0}")]
    DiscordSignatureVerificationFailure(#[from] DiscordSignatureVerificationFailure),
    /// An error from the inner service
    #[error("{0}")]
    InnerError(E),
}

async fn verify_discord_signature(
    public_key: ed25519_dalek::PublicKey,
    signature: &ed25519_dalek::Signature,
    timestamp_bytes: &[u8],
    body_bytes: &[u8],
) -> bool {
    let mut vec = Vec::with_capacity(timestamp_bytes.len() + body_bytes.len());
    vec.extend_from_slice(timestamp_bytes);
    vec.extend_from_slice(body_bytes);
    public_key.verify(&vec, signature).is_ok()
}

fn get_signature(
    headers: &HeaderMap,
) -> Result<ed25519_dalek::Signature, DiscordSignatureVerificationFailure> {
    let header_value = headers.get("X-Signature-Ed25519").ok_or(
        DiscordSignatureVerificationFailure::MissingHeader("X-Signature-Ed25519"),
    )?;
    let header_str = header_value.to_str().map_err(|_| {
        DiscordSignatureVerificationFailure::BadFormat("header must be UTF-8 encoded")
    })?;
    let header_bytes = hex::decode(header_str).map_err(|_| {
        DiscordSignatureVerificationFailure::BadFormat("signature must be in hex format")
    })?;
    let signature = ed25519_dalek::Signature::from_bytes(&header_bytes)
        .map_err(|_| DiscordSignatureVerificationFailure::InvalidSignature)?;

    Ok(signature)
}

fn get_timestamp_bytes(headers: &HeaderMap) -> Result<&[u8], DiscordSignatureVerificationFailure> {
    headers
        .get("X-Signature-Timestamp")
        .map(|header_value| header_value.as_bytes())
        .ok_or(DiscordSignatureVerificationFailure::MissingHeader(
            "X-Signature-Timestamp",
        ))
}

async fn verify_discord_signature_fn(
    public_key: ed25519_dalek::PublicKey,
    request: Request<Bytes>,
) -> Result<Request<Bytes>, DiscordSignatureVerificationFailure> {
    let (mut parts, bytes) = request.into_parts();

    let headers = parts.extract().await?;
    let signature = get_signature(&headers)?;
    let timestamp_bytes = get_timestamp_bytes(&headers)?;

    if !verify_discord_signature(public_key, &signature, timestamp_bytes, &bytes).await {
        return Err(DiscordSignatureVerificationFailure::InvalidSignature);
    }

    parts.headers = headers;
    let request = Request::from_parts(parts, bytes);
    Ok(request)
}

fn verify_discord_signature_layer_fn<S>(
    mut service: S,
) -> impl Service<
    (ed25519_dalek::PublicKey, Request<Bytes>),
    Response = S::Response,
    Error = DiscordSignatureVerificationLayerError<S::Error>,
> + Clone
where
    S: Service<Request<Bytes>> + Clone,
    S::Error: Debug + Display
{
    service_fn(|(public_key, request)| verify_discord_signature_fn(public_key, request)).then(
        |res_request| async move {
            let request = res_request?;
            service
                .ready()
                .await
                .map_err(DiscordSignatureVerificationLayerError::InnerError)?
                .call(request)
                .await
                .map_err(DiscordSignatureVerificationLayerError::InnerError)
        },
    )
}

/// A layer which short-circuits and returns 401 UNAUTHORIZED if the request
/// does not have a valid Discord signature for the timestamp and bytes of
/// the message; otherwise, passes the request on unmodified.
/// Requires that the inner service returns a (StatusCode, JsonValue) so that
/// the short-circuiting produces the same result type, and also that the
/// inner service error implement From<Infallible> (which can be accomplished
/// with the unreachable! macro.
pub fn verify_discord_signature_layer<S>(
    public_key: ed25519_dalek::PublicKey,
) -> impl Layer<
    S,
    Service = impl Service<
        Request<Bytes>,
        Response = S::Response,
        Error = DiscordSignatureVerificationLayerError<S::Error>,
    > + Clone,
>
where
    S: Service<Request<Bytes>> + Clone,
    S::Error: Debug + Display,
{
    layer_fn(move |service: S| {
        ServiceBuilder::new()
            .layer(ClonedStateProviderLayer::new(public_key))
            .layer_fn(verify_discord_signature_layer_fn)
            .service(service)
    })
}
