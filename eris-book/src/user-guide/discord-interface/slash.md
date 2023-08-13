# Slash commands

These commands are used within the context of a specific channel, and usually are specific to that channel.

The available slash commands are:

* **/admin**: instance-level moderator actions. All subcommands require instance admin privileges.
    * **/admin ban + \<URL\>**: Bans an Actor with a specific URL across the entire instance.
    * **/admin channel**: instance-level moderator actions against the channel.
        * **/admin channel block**: Blocks the channel in the instance. 
        * **/admin channel delete**: Deletes the channel. **This cannot be undone.** If the channel is deleted by a Discord guild admin, Eris will see that it has been deleted the next time it tries to send a message in that channel, and will delete it automatically.
        * **/admin channel unblock**: Unblocks the channel if it was previously blocked. (Note that command uses "unblock" rather than "undo block", due to Discord placing a three-keyword limit on command names.)
    * **/admin undo ban + \<URL\>**: Unbans an actor with a specific URL.
    * **/admin user**: instance-level moderator actions against a specific user on the instance. Note that Actors on other instances can be banned (see /admin ban + \<URL\>) from appearing in this instance, but other moderation actions must be performed by their host instance.
        * **/admin user delete + \<user\>**: Deletes the user, and all of their posts. **This cannot be undone.** For a reversible alternative, use /admin user ban.
        * **/admin user ban + \<user\>**: Bans the user from interacting with the instance. This is equivalent to /admin ban + \<URL\>, but specific to instance users.
        * **/admin user unban + \<user\>**: Unbans a local user. This is equivalent to admin undo ban + \<URL\>, but specific to instance users. (Note that this command uses "unban" rather than "undo ban", due to Discord placing a three-keyword limit on command names).
* **/channel**: updates the content stream received by a channel.
    * **/channel block** + \<URL\>: Blocks the Actor with a specific URL. Channels cannot block their parent instance, but any other Actor URL is fine.
    * **/channel follow** + \<URL\>: Attempts to follow the Actor with that URL, which might be on a different ActivityPub network.
    * **/channel undo**: Undoes a previous channel action.
        * **/channel undo block** + \<URL\>: Removes a block on an Actor.
        * **/channel undo follow** + \<URL\>: Stops following an Actor. 
* **/help**: lists the available slash commands
* **/info**: displays general instance info
* **/join**: User joins the instance.
