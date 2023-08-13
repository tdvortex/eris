# Activities

Eris allows different actions to be taken by the instance (Application), users (Persons), or channels (Services).

* **Application**: Delete user, Delete channel, Block Actor, Undo Block Actor, Update instance, Reject Follow
* **User** (local): Create user, Update user, Delete user, Create post, Delete post (aliased as Undo Create post), Update post, Like post, Undo Like post, Announce Object, Undo Announce Object, Block Actor, Undo Block Actor, Accept Follow, Reject Follow
* **Channel**: Create channel, Follow Actor, Undo Follow Actor, Block Actor, Undo Block Actor, Reject Follow

Generally speaking, slash commands (/command in chat) are used for channel management actions; message commands (right-click on a message) are used for post management actions; and user commands (right-click on a user) are used for user management actions. Exceptions apply, however, so see the next pages for details.

Eris will also try to respond to actions taken by verified foreign actors by translating them into one of these actions:

* **Actor** (foreign): Create Object, Update Object, Delete Object, Like post, Undo Like post, Announce Object, Undo Announce Object, Follow user, Undo Follow user, Block user, Undo Block user, Block channel, Undo Block channel, Accept Follow, Reject Follow