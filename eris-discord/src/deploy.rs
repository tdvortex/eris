/// Message commands (right-click on a message)
pub mod message;

/// Slash commands (aka chat input)
pub mod slash;

use std::{fmt::Display, num::NonZeroU64};

use message::message_commands;
pub use slash::slash_commands;
use twilight_http::Client;
use twilight_model::{application::command::Command, id::Id};

/// Sets all slash commands and message commands globally for the app.
/// This is the production deployment--it will be available to all users!
pub async fn set_global_commands(token: impl Display, application_id: NonZeroU64) {
    let commands: Vec<Command> = slash_commands()
        .into_iter()
        .chain(message_commands().into_iter())
        .collect();

    let response = Client::new(format!("Bearer {}", token))
        .interaction(Id::from(application_id))
        .set_global_commands(&commands)
        .await
        .unwrap();

    let status = response.status();

    if !status.is_success() {
        let status_u16 = status.get();

        match response.text().await {
            Ok(text) => panic!(
                "Global command deployment failed, status code: {}, body: {}",
                status_u16, text
            ),
            Err(e) => panic!(
                "Global command deployment failed, status code: {}, but could not decode body for reason {}",
                status_u16, e
            ),
        }
    } else {
        println!("Deployment successful, status code {}", status);
    }
}

/// Sets all slash commands and message commands for a specific guild.
/// This is the test deployment--it will expose these commands only on
/// a specific server.
pub async fn set_guild_commands(token: impl Display, application_id: NonZeroU64, guild_id: NonZeroU64) {
    let commands: Vec<Command> = slash_commands()
        .into_iter()
        .chain(message_commands().into_iter())
        .collect();

    let response = Client::new(format!("Bearer {}", token))
        .interaction(Id::from(application_id))
        .set_guild_commands(Id::from(guild_id), &commands)
        .await
        .unwrap();

    let status = response.status();

    if !status.is_success() {
        let status_u16 = status.get();

        match response.text().await {
            Ok(text) => panic!(
                "Guild deployment to {} failed, status code: {}, body: {}",
                guild_id, status_u16, text
            ),
            Err(e) => panic!(
                "Guild deployment to {} failed, status code: {}, but could not decode body for reason {}",
                guild_id, status_u16, e
            ),
        }
    } else {
        println!("Deployment successful, status code {}", status);
    }
}
