use axum::{response::{Response, IntoResponse}, middleware::Next, extract::State, RequestPartsExt};
use ed25519_dalek::Verifier;
use http::{HeaderMap, StatusCode, Request};
use hyper::body::to_bytes;

const SIGNATURE_REJECTED: (StatusCode, &'static str) = (StatusCode::UNAUTHORIZED, "invalid request signature");

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
    headers.get("X-Signature-Timestamp").map(|header_value| header_value.as_bytes())
}

/// A middleware function which checks that the correct headers for a Discord
/// public key signature are present and valid, given a request with a [`hyper::body::Body`] payload,
/// before passing the request on.
pub async fn verify_discord_signature_hyper(
    State(public_key): State<ed25519_dalek::PublicKey>,
    request: Request<hyper::body::Body>,
    next: Next<hyper::body::Body>,
) -> Response {
    let (mut parts, raw_body) = request.into_parts();
    let Ok(headers): Result<HeaderMap, _> = parts.extract().await else {
        return SIGNATURE_REJECTED.into_response();
    };

    let Some(signature) = get_signature(&headers) else {
        return SIGNATURE_REJECTED.into_response(); 
    };

    let Some(timestamp_bytes) = get_timestamp_bytes(&headers) else {
        return SIGNATURE_REJECTED.into_response();
    };

    let Some(body_bytes) = to_bytes(raw_body).await.ok() else {
        return SIGNATURE_REJECTED.into_response();
    };

    if !verify_discord_signature_inner(public_key, &signature, timestamp_bytes, &body_bytes).await {
        return SIGNATURE_REJECTED.into_response();
    }

    parts.headers = headers;
    let body = hyper::body::Body::from(body_bytes);
    let request = Request::from_parts(parts, body);
    next.run(request).await
}


/// A middleware function which checks that the correct headers for a Discord
/// public key signature are present and valid, given a request with a [`lambda_http::Body`] payload,
/// before passing the request on.
pub async fn verify_discord_signature_lambda(
    State(public_key): State<ed25519_dalek::PublicKey>,
    request: Request<lambda_http::Body>,
    next: Next<lambda_http::Body>,
) -> Response {
    let (mut parts, raw_body) = request.into_parts();
    let Ok(headers): Result<HeaderMap, _> = parts.extract().await else {
        return SIGNATURE_REJECTED.into_response();
    };

    let Some(signature) = get_signature(&headers) else {
        return SIGNATURE_REJECTED.into_response(); 
    };

    let Some(timestamp_bytes) = get_timestamp_bytes(&headers) else {
        return SIGNATURE_REJECTED.into_response();
    };

    if !verify_discord_signature_inner(public_key, &signature, timestamp_bytes, &raw_body).await {
        return SIGNATURE_REJECTED.into_response();
    }

    parts.headers = headers;
    let request = Request::from_parts(parts, raw_body);
    next.run(request).await
}