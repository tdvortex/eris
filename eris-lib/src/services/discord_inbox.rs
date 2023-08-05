use axum::{Router, routing::post, Json, response::IntoResponse};
use twilight_model::application::interaction::Interaction;

use crate::layers::verify_discord_signature_hyper;

async fn post_interaction(Json(_interaction): Json<Interaction>) -> impl IntoResponse {
    todo!()
}

/// Creates an [`axum`] router at the relative path of "/" that responds to 
/// POST requests made by Discord for interactions on application commands.
pub fn discord_router(public_key: ed25519_dalek::PublicKey) -> Router {
    Router::new()
        .route("/", post(post_interaction))
        .route_layer(axum::middleware::from_fn_with_state(
            public_key,
            verify_discord_signature_hyper,
        ))
}