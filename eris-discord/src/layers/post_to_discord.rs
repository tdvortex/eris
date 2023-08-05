use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::message::Embed,
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};

/// An action taken that affects the displayed messages in Discord.
/// Must be carefully throttled to avoid hitting Discord rate limits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag="action")]
pub enum DiscordClientAction<'a> {
    /// Creates a standalone message in a channel.
    #[serde(borrow)]
    CreateMessage(CreateMessage<'a>),
    /// Creates a message replying to another message in that same channel.
    #[serde(borrow)]
    CreateReply(CreateReply<'a>),
    /// Delete a message from a channel. Optionally, may include a reason, which
    /// will be stored in the audit logs for the server.
    #[serde(borrow)]
    DeleteMessage(DeleteMessage<'a>),
    /// Updates the initial response, changing it from the typing indicator to a
    /// proper message.
    #[serde(borrow)]
    UpdateInteractionResponse(UpdateInteractionResponse<'a>),
    /// Updates an existing message. If the message is a reply, will not change
    /// the referenced post.
    #[serde(borrow)]
    UpdateMessage(UpdateMessage<'a>),
}

impl<'a> DiscordClientAction<'a> {
    /// Creates a standalone text message in a channel.
    pub fn create_text_message(channel_id: impl Into<Id<ChannelMarker>>, text: &'a str) -> Self {
        Self::CreateMessage(CreateMessage {
            channel_id: channel_id.into(),
            message: MessagePayload::Text(text),
        })
    }

    /// Creates a standalone embed message in a channel, optionally with some
    /// text outside the embed frame.
    pub fn create_embed_message(
        channel_id: impl Into<Id<ChannelMarker>>,
        embed: impl Into<Embed>,
        text: Option<&'a str>,
    ) -> Self {
        let payload = match text {
            Some(text) => MessagePayload::TextAndEmbed {
                text,
                embed: embed.into(),
            },
            None => MessagePayload::Embed(embed.into()),
        };

        Self::CreateMessage(CreateMessage {
            channel_id: channel_id.into(),
            message: payload,
        })
    }

    /// Replies to a message with text.
    pub fn create_text_reply(
        channel_id: impl Into<Id<ChannelMarker>>,
        replying_to: impl Into<Id<MessageMarker>>,
        text: &'a str,
    ) -> Self {
        Self::CreateReply(CreateReply {
            message_location: MessageLocation {
                channel_id: channel_id.into(),
                message_id: replying_to.into(),
            },
            reply: MessagePayload::Text(text),
        })
    }

    /// Replies to a message with an embed and, optionally, some text outside
    /// the embed frame.
    pub fn create_embed_reply(
        channel_id: impl Into<Id<ChannelMarker>>,
        replying_to: impl Into<Id<MessageMarker>>,
        embed: impl Into<Embed>,
        text: Option<&'a str>,
    ) -> Self {
        let payload = match text {
            Some(text) => MessagePayload::TextAndEmbed {
                text,
                embed: embed.into(),
            },
            None => MessagePayload::Embed(embed.into()),
        };


        Self::CreateReply(CreateReply {
            message_location: MessageLocation {
                channel_id: channel_id.into(),
                message_id: replying_to.into(),
            },
            reply: payload,
        })
    }

    /// Deletes a message and, optionally, puts a reason into the audit log.
    pub fn delete_message(
        channel_id: impl Into<Id<ChannelMarker>>,
        message_id: impl Into<Id<MessageMarker>>,
        reason: Option<&'a str>,
    ) -> Self {
        Self::DeleteMessage(DeleteMessage {
            message_location: MessageLocation {
                channel_id: channel_id.into(),
                message_id: message_id.into(),
            },
            reason,
        })
    }

    /// Reponds to an interaction with a text message.
    pub fn interaction_response_text(
        interaction_token: &'a str,
        text: &'a str,
    ) -> Self {
        Self::UpdateInteractionResponse(UpdateInteractionResponse { interaction_token, response: MessagePayload::Text(text) })
    }

    /// Responds to an interaction with an embed and, optionally, some text
    /// outside the embed frame.
    pub fn interaction_response_embed(
        interaction_token: &'a str,
        embed: impl Into<Embed>,
        text: Option<&'a str>,
    ) -> Self {
        let payload = match text {
            Some(text) => MessagePayload::TextAndEmbed {
                text,
                embed: embed.into(),
            },
            None => MessagePayload::Embed(embed.into()),
        };


        Self::UpdateInteractionResponse(UpdateInteractionResponse { interaction_token, response: payload })
    }

    /// Updates an existing message's body, overwriting it with static text.
    pub fn update_text_message(
        channel_id: impl Into<Id<ChannelMarker>>,
        message_id: impl Into<Id<MessageMarker>>,
        new_text: &'a str
    ) -> Self {
        Self::UpdateMessage(UpdateMessage {
            message_location: MessageLocation { channel_id: channel_id.into(), message_id: message_id.into() },
            message: MessagePayload::Text(new_text)
        })
    }

    /// Updates an existing message's body, overwriting it with an embed and,
    /// optionally, some text outside the embed frame.
    pub fn update_embed_message(
        channel_id: impl Into<Id<ChannelMarker>>,
        message_id: impl Into<Id<MessageMarker>>,
        embed: impl Into<Embed>,
        text: Option<&'a str>,
    ) -> Self {
        let payload = match text {
            Some(text) => MessagePayload::TextAndEmbed {
                text,
                embed: embed.into(),
            },
            None => MessagePayload::Embed(embed.into()),
        };


        Self::UpdateMessage(UpdateMessage {
            message_location: MessageLocation { channel_id: channel_id.into(), message_id: message_id.into() },
            message: payload
        })
    }
}

/// Creates a standalone message in a channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessage<'a> {
    /// The Id of the channel to send the message in.
    pub channel_id: Id<ChannelMarker>,
    /// The payload of the message.
    #[serde(borrow)]
    #[serde(flatten)]
    pub message: MessagePayload<'a>,
}

/// Creates a message replying to another message in that same channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReply<'a> {
    /// The location of the message to be replied to.
    #[serde(flatten)]
    pub message_location: MessageLocation,
    /// The reply message.
    #[serde(borrow)]
    #[serde(flatten)]
    pub reply: MessagePayload<'a>,
}

/// Delete a message from a channel. Optionally, may include a reason, which
/// will be stored in the audit logs for the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMessage<'a> {
    /// The location of the message to be deleted.
    pub message_location: MessageLocation,
    /// A reason for why the post is deleted. This is not displayed openly
    /// but is accessible in the guild audit logs.
    pub reason: Option<&'a str>,
}

/// The location of a message within Discord.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageLocation {
    /// The channel the message was sent in (may be a DM channel).
    pub channel_id: Id<ChannelMarker>,
    /// The message itself.
    pub message_id: Id<MessageMarker>,
}

/// The contents of a message to be posted on Discord.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessagePayload<'a> {
    /// A simple text message.
    #[serde(rename = "content")]
    Text(&'a str),
    /// An Embed, used for Posts.
    Embed(Embed),
    /// An Embed with some extra text outside the Embed frame.
    #[serde(rename = "message")]
    TextAndEmbed {
        /// The text outside the embed frame.
        #[serde(rename = "content")]
        text: &'a str,
        /// The embed.
        embed: Embed,
    }
}

/// Updates the initial response, changing it from the typing indicator to a
/// proper message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInteractionResponse<'a> {
    /// The token for the interaction to respond to.
    pub interaction_token: &'a str,
    /// The message to use as a response.
    #[serde(flatten)]
    pub response: MessagePayload<'a>,
}

/// Updates an existing message. If the message is a reply, will not change
/// the referenced post.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessage<'a> {
    /// The location of the message to be updated.
    #[serde(flatten)]
    pub message_location: MessageLocation,
    /// The new message.
    #[serde(borrow)]
    #[serde(flatten)]
    pub message: MessagePayload<'a>,
}
