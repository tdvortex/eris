#![warn(missing_docs)]
//! eris-discord sets up a service router expose an endpoint to Discord,
//! creates a deployment function for creating the application commands,
//! and supplies utility functions to make calls to the Discord API.

use axum::{response::IntoResponse, routing::post, Json, Router};
use twilight_model::application::interaction::Interaction;

/// Commands to deploy application commands to Discord.
pub mod deploy;

/// [`tower::Layer`]s that can be used to assemble useful Discord-related
/// [`tower::Service`]s.
pub mod layers;

async fn post_interaction(Json(_interaction): Json<Interaction>) -> impl IntoResponse {
    todo!()
}

/// Creates a router at the relative path of "/" that responds to POST requests
/// made by Discord for interactions (both slash and message commands).
pub fn discord_router(public_key: ed25519_dalek::PublicKey) -> Router {
    Router::new()
        .route("/", post(post_interaction))
        .route_layer(axum::middleware::from_fn_with_state(
            public_key,
            layers::verify_discord_signature_hyper,
        ))
}
