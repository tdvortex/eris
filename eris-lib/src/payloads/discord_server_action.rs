use crate::payloads::DiscordClientActionResponse;
use twilight_model::application::interaction::Interaction;

/// Actions that Discord's server might take which may require processing by Eris.
pub enum DiscordServerAction {
    /// Discord made a POST request to our Interactions endpoint.
    PostInteraction(Interaction),
    /// Discord responded to an action taken by the Discord client.
    DiscordClientActionResponse(DiscordClientActionResponse),
}

impl From<Interaction> for DiscordServerAction {
    fn from(value: Interaction) -> Self {
        Self::PostInteraction(value)
    }
}

impl From<DiscordClientActionResponse> for DiscordServerAction {
    fn from(value: DiscordClientActionResponse) -> Self {
        Self::DiscordClientActionResponse(value)
    }
}
