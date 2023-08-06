use std::{convert::Infallible, future::Future, pin::Pin};

use axum::RequestPartsExt;
use ed25519_dalek::Verifier;
use futures_util::{FutureExt, TryFutureExt};
use http::{HeaderMap, Request, StatusCode};
use hyper::{body::to_bytes, Body as HyperBody};
use serde_json::{json, Value as JsonValue};
use thiserror::Error;
use tower::{Layer, Service};

use crate::services::discord_endpoint_real::DiscordEndpointError;


/// A layer which verifies the message signature using Discord's public key to
/// ensure it's authentic.
pub struct DiscordVerificationLayer {
    public_key: ed25519_dalek::PublicKey,
}

impl DiscordVerificationLayer {
    /// Create a new Discord verification layer with Discord's public key
    pub fn new(public_key: ed25519_dalek::PublicKey) -> Self {
        Self { public_key }
    }
}

impl<S> Layer<S> for DiscordVerificationLayer
where
    S: Service<Request<HyperBody>, Response = (StatusCode, JsonValue)> + 'static,
    S::Error: Into<DiscordEndpointError>,
    S: Clone,
{
    type Service = DiscordVerificationService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DiscordVerificationService {
            public_key: self.public_key,
            inner,
        }
    }
}

/// An error that can occur while attempting to verify Discord's signature.
/// This indicates that the signature could not be verified or falsified;
/// a falsified signature should not return an error.
#[derive(Debug, Error)]
pub enum DiscordVerificationServiceError {
    /// An error when using hyper::to_bytes to get the raw bytes of the
    /// request.
    #[error("An error occurred within hyper::to_bytes: {0}")]
    HyperError(#[from] hyper::Error),
}

/// A middleware service which verifies the signature on a [`http::Request`]
/// with a [`hyper::Body`]. If so verified, it passes the request on to the
/// next Service. If falsified, it returns 401 UNAUTHORIZED with a short error
/// Json object. If the request cannot be verified or falsified, returns
/// [DiscordEndpointError::DiscordVerificationService`].
pub struct DiscordVerificationService<S>
where
    S: Service<Request<HyperBody>, Response = (StatusCode, JsonValue)> + 'static,
    S::Error: Into<DiscordEndpointError>,
    S: Clone,
{
    public_key: ed25519_dalek::PublicKey,
    inner: S,
}

async fn verify_discord_signature_inner(
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

fn get_signature(headers: &HeaderMap) -> Option<ed25519_dalek::Signature> {
    let header_value = headers.get("X-Signature-Ed25519")?;
    let header_str = header_value.to_str().ok()?;
    let header_bytes = hex::decode(header_str).ok()?;
    let signature = ed25519_dalek::Signature::from_bytes(&header_bytes).ok()?;

    Some(signature)
}

fn get_timestamp_bytes(headers: &HeaderMap) -> Option<&[u8]> {
    headers
        .get("X-Signature-Timestamp")
        .map(|header_value| header_value.as_bytes())
}

async fn verify_discord_signature(
    public_key: ed25519_dalek::PublicKey,
    request: Request<HyperBody>,
) -> Result<Option<Request<HyperBody>>, DiscordVerificationServiceError> {
    let (mut parts, raw_body) = request.into_parts();

    // parts.extract::<HeaderMap>() has error type Infallible
    // Safe to unwrap
    let headers_res: Result<HeaderMap, Infallible> = parts.extract().await;
    let headers = headers_res.unwrap();

    let Some(signature) = get_signature(&headers) else {
        return Ok(None);
    };

    let Some(timestamp_bytes) = get_timestamp_bytes(&headers) else {
        return Ok(None);
    };

    let body_bytes = to_bytes(raw_body).await?;

    if !verify_discord_signature_inner(public_key, &signature, timestamp_bytes, &body_bytes).await {
        return Ok(None);
    }

    parts.headers = headers;
    let body = hyper::body::Body::from(body_bytes);
    let request = Request::from_parts(parts, body);
    Ok(Some(request))
}

impl<S> Service<Request<HyperBody>> for DiscordVerificationService<S>
where
    S: Service<Request<HyperBody>, Response = (StatusCode, JsonValue)> + 'static,
    S::Error: Into<DiscordEndpointError>,
    S: Clone,
{
    type Response = (StatusCode, JsonValue);

    type Error = DiscordEndpointError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            std::task::Poll::Ready(res) => std::task::Poll::Ready(res.map_err(|e| e.into())),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<HyperBody>) -> Self::Future {
        let public_key = self.public_key;
        // BUG
        let mut inner = self.inner.clone();
        Box::pin(
            verify_discord_signature(public_key, req)
                .err_into::<DiscordEndpointError>()
                .and_then(move |opt_request| {
                    if let Some(request) = opt_request {
                        inner
                            .call(request)
                            .err_into::<DiscordEndpointError>()
                            .left_future()
                    } else {
                        std::future::ready(Ok((
                            StatusCode::UNAUTHORIZED,
                            json!({
                                "error": "signature invalid"
                            }),
                        )))
                        .right_future()
                    }
                }),
        )
    }
}
