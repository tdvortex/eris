# Message commands

These are executed by right-clicking on a message.

While the same command list is shown on all messages, the behavior is different depending on whether the message was an ordinary Discord message or an Embed post message made by Eris on behalf of another Actor.

All of these actions must be invoked on a message in a guild text channel, except for the "Post" commands, which can be taken on messages in a DM to the instance's Application user. This allows users to send a private message to Eris and upgrade it to a post without making the original source message public.

* **Admin**: Instance-level moderator actions against the message's user, or a posts's author.
    * **Ban**: Bans the author of a post or the user of an ordinary message. This does not stop them from posting Discord messages normally, but their Eris posts will not be shown on any channel in the instance.
    * **Undo**: Reverses an admin action.
        * **Ban**: Unbans the author of a post or the user of an ordinary message. 
* **Block**: The user blocks the author of a post or a user of an ordinary message. Posts made by the blocking user will not be forwared to the blocked user, and that user is prevented from liking the blockin user's post. 
* **Channel**: Channel actions regarding the message's user, or a posts' author.
    * **Block**: Blocks the author of a post or a user of an ordinary message. This does not stop them from posting Discord messages normally, but their Eris posts will not be shown on this channel. This is the same as the "/channel block \<author\>" slash command.
    * **Follow**: Follows the author of a post or a user of an ordinary message. This is the same as the "/channel follow \<author\>" slash command.
* **Like**: The user likes a post. 
* **Post**: Actions related to posts made by the user.
    * **Create**: Converts an ordinary Discord message into a post and publishes it to all of the user's followers.
    * **Delete**: Deletes a post that the user is the author of, deleting all of its messages everywhere. *This cannot be undone*.
    * **Update**: Must be used on the original Discord message for a post. Updates all post messages to match the new post.
* **Share**: The user reshares a post with their followers.
* **Undo**: Undoes a previous action.
    * **Block**: The user unblocks the author of a post or a user of an ordinary message.
    * **Channel**: Undoes a channel action.
        * **Block**: Unblocks the author of a post or a user of an ordinary message. This is the same as the "/channel undo block \<author\>" slash command.
        * **Follow**: Unfollows the author of a post or a user of an ordinary message. This is the same as the "/channel undo follow \<author\>" slash command.
    * **Like**: The user removes their like from a post.
    * **Post**: Undoes a post action.
        * **Create**: alias for Post > Delete. *This cannot be undone*.
    * **Share**: The user stops sharing a post. This does *not* delete any messages that were created by the share, however, but it does remove the user from the post's shares collection.