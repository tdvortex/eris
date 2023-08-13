# User commands

These are executed by right-clicking on a user.

* **Admin**: Instance-level moderator actions against the user.
    * **Ban**: Bans the user. This does not stop them from posting Discord messages normally, but their Eris posts will not be shown on any channel in the instance.
    * **Undo**: Reverses an admin action.
        * **Ban**: Unbans the user.
* **Block**: The user executing the command blocks targeted user. Posts made by the blocking user will not be forwared to the blocked user, and that user is prevented from liking the blocking user's post. 
* **Channel**: Channel actions regarding the user.
    * **Block**: Blocks the user in the channel This does not stop them from posting Discord messages normally, but their Eris posts will not be shown on this channel. This is the same as the "/channel block \<user\>" slash command.
    * **Follow**: Follows the user in the channel. This is the same as the "/channel follow \<user\>" slash command.
* **Undo**: Undoes a previous action.  
    * **Block**: The user executing the command unblocks the targeted user.
    * **Channel**: Undoes a channel action.
        * **Block**: Unblocks the user in the channel. This is the same as the "/channel undo block \<user\>" slash command.
        * **Follow**: Unfollows the user in the channel. This is the same as the "/channel undo follow \<user\>" slash command.