/// Message commands (right-click on a message)
pub mod message;

/// Slash commands (aka chat input)
pub mod slash;

use std::{fmt::Display, num::NonZeroU64};

pub use message::message_commands;
pub use slash::slash_commands;
use twilight_http::Client;
use twilight_model::{application::command::Command, id::Id};

/// Sets all slash commands and message commands globally for the app.
/// This is the production deployment--it will be available to all users!
pub async fn set_global_commands(token: impl Display, application_id: NonZeroU64) {
    let commands: Vec<Command> = slash_commands().chain(message_commands()).collect();

    let response = Client::new(format!("Bearer {token}"))
        .interaction(Id::from(application_id))
        .set_global_commands(&commands)
        .await
        .unwrap();

    let status = response.status();

    if !status.is_success() {
        let status_u16 = status.get();

        match response.text().await {
            Ok(text) => panic!(
                "Global command deployment failed, status code: {status_u16}, body: {text}"),
            Err(e) => panic!(
                "Global command deployment failed, status code: {status_u16}, but could not decode body for reason {e}"),
        }
    } else {
        println!("Deployment successful, status code {status}");
    }
}

/// Sets all slash commands and message commands for a specific guild.
/// This is the test deployment--it will expose these commands only on
/// a specific server.
pub async fn set_guild_commands(
    token: impl Display,
    application_id: NonZeroU64,
    guild_id: NonZeroU64,
) {
    let commands: Vec<Command> = slash_commands().chain(message_commands()).collect();

    let response = Client::new(format!("Bearer {token}"))
        .interaction(Id::from(application_id))
        .set_guild_commands(Id::from(guild_id), &commands)
        .await
        .unwrap();

    let status = response.status();

    if !status.is_success() {
        let status_u16 = status.get();

        match response.text().await {
            Ok(text) => panic!(
                "Guild deployment to {guild_id} failed, status code: {status_u16}, body: {text}"),
            Err(e) => panic!(
                "Guild deployment to {guild_id} failed, status code: {status_u16}, but could not decode body for reason {e}"),
        }
    } else {
        println!("Deployment successful, status code {status}");
    }
}
