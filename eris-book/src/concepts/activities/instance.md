# Instance Activities

These actions are taken by the Application. These are, either explicitly or implictly, admin actions.

## Delete user

The instance deletes a user, and all of their posts, and all attachments to those posts. Any attempt to access these objects in the future will give a Tombstone object.

**This action cannot be undone**. Use with extreme caution! For a reversible alternative, use Block Actor.

"Instance Delete Person" is the admin version of this Activity, indicating that the account has been deleted by an instance admin. Users can also delete their own accounts, which are recorded as "Person Delete Person" instead.

This is triggered by the "Admin > Delete user" message command (right-click in Discord). This can also be executed with a DELETE request to "/users/{user_id}", or by using the "deleteUser" mutation in GraphQL. 

This action requires a verified Discord admin user session.

## Delete Channel

The instance deletes a channel's Service actor, replacing it with a Tombstone. This doesn't actually delete the channel itself in Discord (that has to be done by that guild's admins), but it does delete all records of messages that were posted in the channel, and the channel stops following any user it was previously following.

This is triggered by the "/delete channel" slash command in Discord. It can also be executed by a DELETE request to "/channels/{channel_id}", or using the "deleteChannel" mutation in GraphQL. 

This action requires a verified Discord admin user session.

**This action cannot be undone**. Use with extreme caution!

## Block Actor

The instance blocks an actor, banning them. This is behaves similarly as if every channel and every user on the instance all blocked the actor. If the actor is local, then any activities they attempt will be rejected, with the exception of "delete user" (a banned user is allowed to delete their own account).

This does *not* delete their posts, or change their following/followed by status. If the user is later unbanned, they will be restored to full functionality within the instance.

An instance cannot block itself, but it can block other instances, refusing all incoming activities and not forwarding any outgoing activities.

This is triggered by the "Ban user" message command in Discord. It can also be triggered by a POST request to "/banned" with the URL of the actor to be banned, or using the "banUser" mutation in GraphQL.

This action requires a verified Discord admin user session.

## Undo Block Actor

The instance unblocks an actor, unbanning them. This removes the block on their profile, restoring them to full functionality.

This is triggered by the "Admin > Unban user" message command in Discord. It can also be triggered by a POST request to "/unbanned" with the URL of the actor to be banned, or using the "unbanUser" mutation in GraphQL.

This action requires a verified Discord admin user session.

## Update instance

The instance's settings can be changed with this action.

There is no Discord application command to update the entire instance's settings, but it can be performed by a PUT request to "/" or using the "updateInstance" mutation in graphQL.

The instance Applications's private key **cannot** be changed. Doing so would make impossible for other instances to verify its actions.

This action requires a verified Discord admin user session.

## Reject follow

If a foreign actor attempts to Follow an instance, it will be automatically responded with Reject Follow. Instances cannot be followed.