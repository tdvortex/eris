# User Activities

These are actions taken by a verified Discord user.

## Create user

The user creates a new Person profile for themselves.

This is triggered by a Discord user executing "/join" slash command. If the instance's enrollment is set to "open", and the user doesn't already have a profile on the instance, then they will be prompted to pick a WebFinger handle, which must be unique on the instance. If the instance's enrollment is set to "closed", the request will be rejected.

This can also be executed with a POST request to "/users", or by using the "createUser" mutation in GraphQL. 

## Update user

The user changes their personal settings.

There is no single "update" command in Discord, but individual management actions are covered by the "Profile > Update" message command group (right-click on user). 

This can also be executed with a PUT request to "/users/{user_id}", or by using the "updateProfile" mutation in GraphQL.

## Delete user

The instance deletes a user, and all of their posts, and all attachments to those posts. Any attempt to access these objects in the future will give a Tombstone object.

**This action cannot be undone**. Use with extreme caution!

"Person Delete Person" is the user-initiated version of this Activity, indicating that the user has voluntarily deleted their own account". "Instance Delete Person" is the admin version of this Activity, indicating that the account has been deleted by an instance admin. 

This is triggered by the "Profile > Delete user" either as a message command on a post by that user, or user command in Discord. This can also be executed with a DELETE request to "/users/{user_id}", or by using the "deleteUser" mutation in GraphQL. 

This action requires a verified Discord admin user session.


## Create post

The user makes a new post, which is automatically shared with all of their followers.

This is executed by using the "Post" message command (right-click on message) on a non-post message in Discord, and only functions if the message was created by the user taking the action.

If the posted message has an image or video attachment, a corresponding Image or Video object is created, and the ID of the created object is listed in the attachments field of the Note object.

New posts can also be made directly by a POST request to "/users/{user_id}/posts", or by the "createPost" mutation in GraphQL.

## Delete post (or Undo Create Post).

The user deletes an existing post, which deletes the message from all channels, and notifies anyone who has liked or shared the post that it has been deleted.

This is executed by the "Delete post" message command (right-click on message) on a post embed message in Discord, and only functions if the message was either created by the user taking the action, or the user is an admin of the instance.

It can also be executed by a DELETE request to "/users/{user_id}/posts/{post_id}", or by the "deletePost" mutation in GraphQL.

**This action cannot be undone**.

## Update post

The user updates an existing post, which updates the message from all channels, and notifies anyone who has liked or shared the post that it has been modified.

This is exeucted by the "Update post" message command (right click on message) on the **original** source message of a post in Discord. Eris will reread the message and update the post accordingly.

It can also be executed by a PUT request to "/users/{user_id}/posts/{post_id}", or by the "updatePost" mutation in GraphQL.

## Like Object

The user Likes a post, or an external object that has been posted to a channel. This adds the object's ID to the user's "liked" collection, and notifies the instance which owns the object that it has been liked, with the expectation (but not the requirement) that the user's ID is added to the object's "likes" collection. 

Unlike Follows (which require confirmation to complete), Likes are fire-and-forget. This can result in asymmetry where a user likes an object but the object does not recognize that it has been liked.

This is executed by the "Like post" message command (right click on a message) on a post Embed message in Discord. It can also be executed by a POST request with the Id of the liked Object to "/users/{user_id}/liked/", or by the "likeObject" mutation in GraphQL.

## Undo Like Object

The user removes their Like from a post, or an external object that has been posted to a channel. This removes the object's ID from the user's "liked" collection, and notifies the instance which owns the object that it is no longer liked, with the expectation (but not the requirement) that the user's ID is removed from the object's "likes" collection. 

This is executed by the "Undo > Like post" message command (right click on a message) on a post Embed message in Discord. It can also be executed by a POST request with the Id of the un-liked Object to "/users/{user_id}/unlike/", or by the "unlikeObject" mutation in GraphQL.

## Announce Object

The user reshares an Object as a post. The object's owning instance is notified, with the expectation (but not the requirement) that the user's ID is added to the object's "shares" collection. Additionally, all of the user's followers are notified.

When an Eris channel receives an incoming Announce Object activity, it will post that object as a message *only if* that object has not been posted before. Only the first time an object is posted, with either Create Object or Announce Object, will a new message be created.

This is executed by the "Share post" message command (right click on a message) on a post Embed message in Discord. It can also be executed by a POST request with the Id of the shared Object to "/users/{user_id}/share", or by the "shareObject" mutation in GraphQL.

## Undo Announce Object

The user stops sharing an Object. The object's owning instance is notified, with the expectation (but not the requirement) that the user's ID is removed from the object's "shares" collection. 

Unlike "Delete post", this does **not** delete the message from any channels where it was already posted. It may have other effects on other non-Eris applications.

This is executed by the "Undo > Share post" message command (right click on a message) on a post Embed message in Discord. It can also be executed by a POST request with the Id of the un-shared object to "/users/{user_id}/unshare", or by the "unshareObject" mutation in GraphQL.


## Block Actor

The user blocks an actor. **That actor is not notified**, (in keeping with the SHOULD NOT directive in [the ActivityPub spec](https://www.w3.org/TR/activitypub/#block-activity-outbox)), and a user's blocks are **never public**. 

After blocking, any Activities the user takes are not delivered to the blocked target. If the blocked actor is a local Eris channel, then any posts the user makes will not be shown on that channel. 

However, if the blocked target is on a different instance or application, there is no guarantee that a user's posts won't be shown to them anyway via reshares from unblocked third parties.

Additionally, the blocked target is prevented from modifying the "likes" or "shares" of any Notes the user made, but this doesn't necessarily prevent them from actually resharing the post. 

This is executed by "Block user" either as a message command on a post by that user, or user command in Discord. It can also be executed by a POST request with the Id of the blocked actor to "/users/{user_id}/block", or by the "blockActor" mutation in GraphQL.

## Undo Block Actor

The user unblocks an actor. **That actor is not notified**, (in keeping with the SHOULD NOT directive in [the ActivityPub spec](https://www.w3.org/TR/activitypub/#block-activity-outbox)), and a user's unblocks are **never public**. 

After unblocking, the user appears as normal to the unblocked actor. If the unblocked actor is a local channel, then their posts will once again be permitted to be shown, but will not retroactively be shown.

This is executed by the "Undo > Block user (as user)" either as a message command on a post by that user, or user command in Discord. It can also be executed by a POST request with the Id of the unblocked actor to "/users/{user_id}/unblock", or by the "unblockActor" mutation in GraphQL.

## Accept Follow

When another Actor (either an Eris user or a foreign Actor) sends a Follow Person activity, if the user's profile is set to "accept_follows = true" (the default), then Eris will automatically respond with an Accept Follow request and add the follower's Id to the user's followers Collection.

## Reject Follow

When another Actor (either an Eris user or a foreign Actor) sends a Follow Person activity, if the user's profile is set to "accept_follows = false", then Eris will automatically respond with a Reject Follow request and add the follower's Id to the user's followers Collection.