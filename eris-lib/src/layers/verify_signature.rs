use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    ops::Deref,
};

use axum::response::IntoResponse;
use ed25519_dalek::Verifier;
use futures_util::{
    future::{ready, Either, Ready},
    FutureExt,
};
use http::{HeaderMap, Request};
use hyper::body::Bytes;
use thiserror::Error;
use tower::{layer::layer_fn, Layer, Service, ServiceBuilder, ServiceExt};

/// Wrapper around an [ed25519_dalek::PublicKey]. This makes the key
/// immutable once defined, and give an opaque type to be used as
/// an HTTP extension.
#[derive(Debug, Clone, Copy)]
pub struct DiscordPublicKey(ed25519_dalek::PublicKey);

impl Deref for DiscordPublicKey {
    type Target = ed25519_dalek::PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ed25519_dalek::PublicKey> for DiscordPublicKey {
    fn from(value: ed25519_dalek::PublicKey) -> Self {
        Self(value)
    }
}

/// A reason for the signature validation failure
#[derive(Debug, Error)]
pub enum DiscordSignatureVerificationFailure {
    /// Expected extension with Discord public key
    #[error("Missing Discord public key in extensions")]
    MissingPublicKey,
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
    /// Missing the public key extension

    /// Signature verification failed
    #[error("{0}")]
    DiscordSignatureVerificationFailure(#[from] DiscordSignatureVerificationFailure),
    /// An error from the inner service
    #[error("{0}")]
    InnerError(E),
}

fn valid_discord_signature(
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

fn verify_discord_signature(
    request: Request<Bytes>,
) -> Result<Request<Bytes>, DiscordSignatureVerificationFailure> {
    let (parts, bytes) = request.into_parts();
    let public_key = *parts
        .extensions
        .get::<DiscordPublicKey>()
        .ok_or(DiscordSignatureVerificationFailure::MissingPublicKey)?;
    let signature = get_signature(&parts.headers)?;
    let timestamp_bytes = get_timestamp_bytes(&parts.headers)?;

    if !valid_discord_signature(*public_key, &signature, timestamp_bytes, &bytes) {
        return Err(DiscordSignatureVerificationFailure::InvalidSignature);
    }

    let request = Request::from_parts(parts, bytes);
    Ok(request)
}
#[derive(Clone)]
struct DiscordSignatureVerificationService<S>
where
    S: Clone,
{
    inner: S,
}

impl<S, E> Service<Request<Bytes>> for DiscordSignatureVerificationService<S>
where
    S: Service<Request<Bytes>, Error = DiscordSignatureVerificationLayerError<E>> + Clone,
    S::Response: IntoResponse,
    E: Debug + Display,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = Either<S::Future, Ready<Result<S::Response, S::Error>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Bytes>) -> Self::Future {
        match verify_discord_signature(req) {
            Ok(request) => self.inner.call(request).left_future(),
            Err(e) => ready(Err(
                DiscordSignatureVerificationLayerError::DiscordSignatureVerificationFailure(e),
            ))
            .right_future(),
        }
    }
}

/// A layer which verifies all incoming requests to make sure they are
/// signed with Discord's public key for validity. If signature is missing
/// or invalid, returns an error to prevent any further handling by inner
/// layers.
pub fn verify_discord_signature_layer<S>(
    public_key: ed25519_dalek::PublicKey,
) -> impl Layer<
    S,
    Service = impl Service<
        Request<Bytes>,
        Response = axum::response::Response,
        Error = DiscordSignatureVerificationLayerError<S::Error>,
    > + Clone,
>
where
    S: Service<Request<Bytes>> + Clone,
    S::Response: IntoResponse,
    S::Error: Debug + Display,
{
    layer_fn(move |service: S| {
        ServiceBuilder::new()
            .layer(tower_http::add_extension::AddExtensionLayer::new(
                public_key,
            ))
            .service(DiscordSignatureVerificationService {
                inner: service
                    .map_err(DiscordSignatureVerificationLayerError::InnerError)
                    .map_response(IntoResponse::into_response),
            })
    })
}
