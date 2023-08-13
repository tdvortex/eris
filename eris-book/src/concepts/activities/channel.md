# Channel Activities

Unlike standard user and admin permissions, authorization privileges for channels are special. Whether a user has permissions to view a channel, modify a channel, or use application commands is dictated by local guild (Discord server) permissions; **not by Eris**.

Eris cannot monitor when guild permissions change to know when channel access privileges change, so it cannot effectively mediate over a REST or GraphQL API, even for a signed-in and authenticated user.

As a result, channel actions can **only** be performed from within Discord. 
## Create channel

A new channel is created, as a Service Actor.

This is triggered automatically the first time any Eris slash command is used in a Discord channel. It can be disabled by setting "allow_new_channels" to false in the instance settings.

## Follow Actor

The channel attempts to follow another Actor. This is executed by either the "/channel follow" slash command, or by the "Channel > Follow" message command on an Embed message in a channel, or by "Channel > Follow" user command on a user in a channel.

This triggers a Follow activity, which is then sent to the instance that user belongs to. If that instance sends back an Accept Follow message, the channel will then follow that user, and their Create and Announce activities should be automatically forwarded to the channel. If the instance sends back a Reject Follow message (or doesn't send back a response at all) then the follow will not be executed.

## Undo Follow Actor

The channel stops following another Actor. This is execute by either the "/channel unfollow" slash command, or by the "Channel > Undo > Follow" message command on an Embed message in a channel, or by "Channel > Undo > Follow" user command on a user in a channel.

This sends an Undo Follow message to the source instance for that actor, with the expectation (but not the requirement) that that instance will stop forwarding activities to the channel.

## Block Actor

The channel blocks another Actor. This is executed by either the "/channel block" slash command, or by the "Channel > Block" message command on an Embed message in a channel, or by "Channel > Block" user command on a user in a channel.

**That actor is not notified**, (in keeping with the SHOULD NOT directive in [the ActivityPub spec](https://www.w3.org/TR/activitypub/#block-activity-outbox)), and a channel's blocks are **never public**. 

If an actor is blocked by a channel, that channel will ignore any received Activities, and any Objects which are Created or Announced with that actor as their original source. 

## Unblock Actor

The channel stops blocking another Actor. This is executed by either the "/channel undo block" slash command, or by the "Channel > Undo > Block" message command on an Embed message in a channel, or by "Channel > Undo > Block" user command on a user in a channel.

**That actor is not notified**, (in keeping with the SHOULD NOT directive in [the ActivityPub spec](https://www.w3.org/TR/activitypub/#block-activity-outbox)), and a channel's unblocks are **never public**. .

If an actor is unblocked by a channel, the channel will allow their Activities and posted Objects to be shown as normal.

## Reject follow

If a foreign actor attempts to Follow a channel, it will be automatically responded with Reject Follow. Channels cannot be followed.