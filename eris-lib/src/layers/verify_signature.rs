use std::convert::Infallible;

use axum::RequestPartsExt;
use ed25519_dalek::Verifier;
use http::{HeaderMap, Request, StatusCode};
use hyper::body::Bytes;
use serde_json::{json, Value as JsonValue};
use tower::{layer::layer_fn, service_fn, Layer, Service, ServiceBuilder, ServiceExt};

use super::provide_cloned_state::ClonedStateProviderLayer;

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

async fn verify_discord_signature_fn(
    public_key: ed25519_dalek::PublicKey,
    request: Request<Bytes>,
) -> Result<Option<Request<Bytes>>, Infallible> {
    let (mut parts, bytes) = request.into_parts();

    let headers = parts.extract().await?;

    let Some(signature) = get_signature(&headers) else {
        return Ok(None);
    };

    let Some(timestamp_bytes) = get_timestamp_bytes(&headers) else {
        return Ok(None);
    };

    if !verify_discord_signature(public_key, &signature, timestamp_bytes, &bytes).await {
        return Ok(None);
    }

    parts.headers = headers;
    let request = Request::from_parts(parts, bytes);
    Ok(Some(request))
}

fn verify_discord_signature_layer_fn<S>(
    mut service: S,
) -> impl Service<
    (ed25519_dalek::PublicKey, Request<Bytes>),
    Response = (StatusCode, JsonValue),
    Error = S::Error,
> + Clone
where
    S: Service<Request<Bytes>, Response = (StatusCode, JsonValue)> + Clone,
    S::Error: From<Infallible>,
{
    service_fn(|(public_key, request)| verify_discord_signature_fn(public_key, request)).then(
        |res_opt_request| async move {
            match res_opt_request {
                Ok(Some(request)) => service.call(request).await,
                Ok(None) => Ok((
                    StatusCode::UNAUTHORIZED,
                    json!({
                        "error": "invalid signature"
                    }),
                )),
                Err(_infallible) => unreachable!(),
            }
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
    Service = impl Service<Request<Bytes>, Response = (StatusCode, JsonValue), Error = S::Error> + Clone,
>
where
    S: Service<Request<Bytes>, Response = (StatusCode, JsonValue)> + Clone,
    S::Error: From<Infallible>,
{
    layer_fn(move |service: S| {
        ServiceBuilder::new()
            .layer(ClonedStateProviderLayer::new(public_key))
            .layer_fn(verify_discord_signature_layer_fn)
            .service(service)
    })
}
