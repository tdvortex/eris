#![warn(missing_docs)]
//! eris-discord sets up a service router expose an endpoint to Discord,
//! creates a deployment function for creating the application commands,
//! and supplies utility functions to make calls to the Discord API.

use axum::{
    extract::{RawBody, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use ed25519_dalek::Verifier;
use hyper::{body::to_bytes, StatusCode};
use twilight_model::application::interaction::Interaction;

/// Module for deploying application commands to Discord.
pub mod deploy;

async fn verify_discord_signature(
    public_key: ed25519_dalek::PublicKey,
    headers: &HeaderMap,
    body_str: &str,
) -> bool {
    // Verify discord interaction signature
    // Check that "X-Signature-Ed25519" header is present
    let Some(header_value) = headers.get("X-Signature-Ed25519") else {
        return false;
    };
    // Check that it is a value utf-8 string
    let Ok(header_str) = header_value.to_str() else {
        return false;
    };
    // Check that the utf-8 string decodes to a byte vec
    let Ok(bytes) = hex::decode(header_str) else {
        return false;
    };
    // Check that the bytes are a valid ed25519 signature
    let Ok(signature) = ed25519_dalek::Signature::from_bytes(&bytes) else {
        return false;
    };

    // Check that "X-Signature-Timestamp" header is present
    let Some(header_value) = headers.get("X-Signature-Timestamp") else {
        return false;
    };

    // Check that it is a value utf-8 string
    let Ok(timestamp_str) = header_value.to_str() else {
        return false;
    };

    // Concatenate the timestamp and the body into a single UTF-8 string
    let joined_string = format!("{}{}", timestamp_str, body_str);

    // Convert the joined utf-8 string back to bytes and verify it using the
    // public key and signature
    public_key
        .verify(joined_string.as_bytes(), &signature)
        .is_ok()
}

/// Persistent state needed to respond to Discord interactions.
#[derive(Debug, Clone)]
pub struct DiscordInteractionState {
    /// This application's public key, fromthe Discord developer portal
    pub discord_public_key: ed25519_dalek::PublicKey,
}

#[axum_macros::debug_handler]
async fn post_discord(
    State(state): State<DiscordInteractionState>,
    headers: HeaderMap,
    RawBody(raw_body): RawBody,
) -> Response {
    let public_key = state.discord_public_key;
    let Some(body_bytes) = to_bytes(raw_body).await.ok() else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "could not retrieve request bytes").into_response();
    };
    let Ok(body_str) = std::str::from_utf8(&body_bytes) else {
        return (StatusCode::BAD_REQUEST, "request body is not valid utf-8").into_response();
    };

    if !verify_discord_signature(public_key, &headers, body_str).await {
        return (StatusCode::UNAUTHORIZED, "invalid request signature").into_response();
    }

    let Ok(interaction) = serde_json::from_str(body_str) else {
        return (StatusCode::OK, "could not parse interaction").into_response();
    };

    post_interaction(&interaction).await.into_response()
}

async fn post_interaction(_interaction: &Interaction) -> impl IntoResponse {
    todo!()
}

/// Creates a router at the relative path of "/" that responds to POST requests
/// made by Discord for interactions (both slash and message commands).
pub fn discord_router(state: DiscordInteractionState) -> Router {
    Router::new()
        .route("/", post(post_discord))
        .with_state(state)
}