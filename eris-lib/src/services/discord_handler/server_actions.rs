use twilight_model::application::interaction::Interaction;

use crate::services::discord_client::client_action_response::ClientActionResponse;

/// Actions that Discord's server might take which may require processing by Eris.
pub enum DiscordServerAction {
    /// Discord made a POST request to our Interactions endpoint.
    PostInteraction(Interaction),
    /// Discord responded to an action taken by the Discord client.
    ClientActionResponse(ClientActionResponse),
}

