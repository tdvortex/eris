use twilight_model::application::command::{Command, CommandType};
use twilight_model::guild::Permissions;
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::command::StringBuilder as StringOptionBuilder;

/// Slash command for /block <url>.
/// Requires permissions to manage messages, because this will prevent some
/// messages from appearing in this channel that otherwise would have appeared.
pub fn block() -> Command {
    CommandBuilder::new(
        "block",
        "Block all posts from an actor from appearing in this channel",
        CommandType::ChatInput,
    )
    .option(
        StringOptionBuilder::new("url", "The URL of the actor to block")
            .autocomplete(false)
            .required(true),
    )
    .dm_permission(false)
    .default_member_permissions(Permissions::USE_SLASH_COMMANDS.union(Permissions::MANAGE_MESSAGES))
    .validate()
    .unwrap()
    .build()
}

/// Slash command for /follow <url>.
/// Requires permissions to send messages with links and files, because this
/// will follow posts that are sent to this channel which may have links/files.
pub fn follow() -> Command {
    CommandBuilder::new(
        "follow",
        "Follow an actor in this channel",
        CommandType::ChatInput,
    )
    .option(
        StringOptionBuilder::new("url", "The URL of the actor to follow")
            .autocomplete(false)
            .required(true),
    )
    .dm_permission(false)
    .default_member_permissions(
        Permissions::USE_SLASH_COMMANDS
            .union(Permissions::SEND_MESSAGES)
            .union(Permissions::EMBED_LINKS)
            .union(Permissions::ATTACH_FILES),
    )
    .validate()
    .unwrap()
    .build()
}

/// Slash command for /unblock <url>.
/// Requires all of the permissions for both block and follow,
/// since this will undo a block and permit new messages to appear.
pub fn unblock() -> Command {
    CommandBuilder::new(
        "unblock",
        "Unblock an actor, allowing their posts to be shown in this channel",
        CommandType::ChatInput,
    )
    .option(
        StringOptionBuilder::new("url", "The URL of the actor to unblock")
            .autocomplete(false)
            .required(true),
    )
    .dm_permission(false)
    .default_member_permissions(
        Permissions::USE_SLASH_COMMANDS
            .union(Permissions::MANAGE_MESSAGES)
            .union(Permissions::SEND_MESSAGES)
            .union(Permissions::EMBED_LINKS)
            .union(Permissions::ATTACH_FILES),
    )
    .validate()
    .unwrap()
    .build()
}

/// Slash command for /unfollow <url>.
/// Requires permissions to manage messages, because this will prevent some
/// messages from appearing in this channel that otherwise would have appeared.
pub fn unfollow() -> Command {
    CommandBuilder::new(
        "unfollow",
        "Unfollow an actor in this channel. Their posts will still appear if shared.",
        CommandType::ChatInput,
    )
    .option(
        StringOptionBuilder::new("url", "The URL of the actor to unfollow")
            .autocomplete(false)
            .required(true),
    )
    .dm_permission(false)
    .default_member_permissions(Permissions::USE_SLASH_COMMANDS.union(Permissions::MANAGE_MESSAGES))
    .validate()
    .unwrap()
    .build()
}

/// An iterator that produces these slash commands:
/// /block <url>
/// /follow <url>
/// /unblock <url>
/// /unfollow <url>
pub fn slash_commands() -> impl ExactSizeIterator<Item = Command> {
    vec![block(), follow(), unblock(), unfollow()].into_iter()
}
