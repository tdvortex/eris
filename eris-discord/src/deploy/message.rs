use twilight_model::application::command::Command;

pub fn block_in_channel() -> Command {}

pub fn delete_post() -> Command {}

pub fn follow_in_channel() -> Command {}

pub fn like() -> Command {}

pub fn post() -> Command {}

pub fn share() -> Command {}

pub fn unblock_in_channel() -> Command {}

pub fn unfollow_in_channel() -> Command {}

pub fn unlike() -> Command {}

pub fn unshare() -> Command {}


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
/// "Unshare"
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
        unshare(),
    ].into_iter()
}