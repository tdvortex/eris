# Slash commands

These commands are used within the context of a specific channel, and usually are specific to that channel.

The available slash commands are:

* **/admin**: instance-level moderator actions. All subcommands require instance admin privileges.
    * **/admin ban + \<URL\>**: Bans an Actor with a specific URL across the entire instance.
    * **/admin channel**: instance-level moderator actions against the channel.
        * **/admin channel block**: Blocks the channel in the instance. 
        * **/admin channel delete**: Deletes the channel. *This cannot be undone.* If the channel is deleted by a Discord guild admin, Eris will see that it has been deleted the next time it tries to send a message in that channel, and will delete it automatically.
        * **/admin channel unblock**: Unblocks the channel if it was previously blocked. (Note that this is the only command with "unblock" rather than "undo block", due to Discord placing a three-keyword limit on command names.)
    * **/admin undo ban + \<URL\>**: Unbans an actor with a specific URL.
* **/channel**: updates the content stream received by a channel.
    * **/channel block** + \<URL\>: Blocks the Actor with a specific URL. Channels cannot block their parent instance, but any other Actor URL is fine.
    * **/channel follow** + \<URL\>: Attempts to follow the Actor with that URL, which might be on a different ActivityPub network.
    * **/channel undo**: Undoes a previous channel action.
        * **/channel undo block** + \<URL\>: Removes a block on an Actor.
        * **/channel undo follow** + \<URL\>: Stops following an Actor. 
* **/help**: lists the available slash commands
* **/info**: displays general instance info
* **/join**: User joins the instance.
