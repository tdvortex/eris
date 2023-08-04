use twilight_model::{application::command::{Command, CommandType}, guild::Permissions};
use twilight_util::builder::command::CommandBuilder;

/// Message command alternative to [`crate::deploy::slash::block`]
pub fn block_in_channel() -> Command {
    CommandBuilder::new("Block in channel", "Block the original poster of this message in this channel", CommandType::Message)
    .dm_permission(false)
    .default_member_permissions(Permissions::USE_SLASH_COMMANDS.union(Permissions::MANAGE_MESSAGES))
    .validate()
    .unwrap()
    .build()
}

/// Deletes a post. Fails if used on anything other than a post 
/// attributed to the person using the command.
pub fn delete_post() -> Command {
    CommandBuilder::new("Delete post", "Delete your post (on all channels)", CommandType::Message)
    .dm_permission(true)
    .validate()
    .unwrap()
    .build()
}

/// Message command alternative to [`crate::deploy::slash::follow`]
pub fn follow_in_channel() -> Command {
    CommandBuilder::new("Follow in channel", "Follow the original poster of this message in this channel", CommandType::Message)
    .dm_permission(false)
    .default_member_permissions(Permissions::USE_SLASH_COMMANDS
        .union(Permissions::SEND_MESSAGES)
        .union(Permissions::EMBED_LINKS)
        .union(Permissions::ATTACH_FILES))
    .validate()
    .unwrap()
    .build()
}

/// Likes a post. Fails if used on a message that is not a post.
pub fn like() -> Command {
    CommandBuilder::new("Like", "Like this post", CommandType::Message)
    .dm_permission(true)
    .validate()
    .unwrap()
    .build()
}

/// Upgrades a Discord message into a post. Fails if used on a message
/// not originally posted by the person using the command.
pub fn post() -> Command {
    CommandBuilder::new("Post", "Upgrade this message into a post", CommandType::Message)
    .dm_permission(true)
    .validate()
    .unwrap()
    .build()
}

/// Shares the post. Fails if used on a message that is not a post.
pub fn share() -> Command {
    CommandBuilder::new("Share", "Share this post with your followers", CommandType::Message)
    .dm_permission(true)
    .validate()
    .unwrap()
    .build()
}

/// Message command alternative to [`crate::deploy::slash::unblock`]
pub fn unblock_in_channel() -> Command {
    CommandBuilder::new("Unblock in channel", "Unblock the original poster of this message in this channel", CommandType::Message)
    .dm_permission(false)
    .default_member_permissions(Permissions::USE_SLASH_COMMANDS
        .union(Permissions::MANAGE_MESSAGES)
        .union(Permissions::SEND_MESSAGES)
        .union(Permissions::EMBED_LINKS)
        .union(Permissions::ATTACH_FILES))
    .validate()
    .unwrap()
    .build()
}

/// Message command alternative to [`crate::deploy::slash::unfollow`]
pub fn unfollow_in_channel() -> Command {
    CommandBuilder::new("Unfollow in channel", "Stop following the original poster of this message in this channel", CommandType::Message)
    .dm_permission(false)
    .default_member_permissions(Permissions::USE_SLASH_COMMANDS.union(Permissions::MANAGE_MESSAGES))
    .validate()
    .unwrap()
    .build()
}

/// Removes a like from a post. Fails if it was not a post the user had
/// previously liked.
pub fn unlike() -> Command {
    CommandBuilder::new("Like", "Stop liking this post", CommandType::Message)
    .dm_permission(true)
    .validate()
    .unwrap()
    .build()
}


/// Message commands (right-click on message) for the following user actions:
/// "Block in channel"
/// "Delete Post"
/// "Follow in channel"
/// "Like"
/// "Post"
/// "Share"
/// "Unblock in channel"
/// "Unfollow in channel"
/// "Unlike"
pub fn message_commands() -> impl ExactSizeIterator<Item = Command> {
    vec![
        block_in_channel(),
        delete_post(),
        follow_in_channel(),
        like(),
        post(),
        share(),
        unblock_in_channel(),
        unfollow_in_channel(),
        unlike(),
    ].into_iter()
}