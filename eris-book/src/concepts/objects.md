# Objects

Eris uses these conceptual entities, most of which are ActivityPub Objects.

## Local entities

These are the "things" that Eris will create and present to the Fediverse.

### Instance/Application

An Eris *instance* is one server, with one web domain, tied to a specific Discord "Application" using their [Application Commands](https://discord.com/developers/docs/interactions/application-commands) API. 

It is an ActivityPub Appliciation. It is an Actor, meaning it can create Activities. These are actions which relate to the operation of the instance itself, including user management and channel management. For example, when a new user joins, this is handled as a Create Person activity.

The relative path for an instance is just the root domain, "/".

### User/Person

An Eris *user* is a Discord user that has additionally signed up with an Eris instance using the "/join" slash command. 

They are an ActivityPub Person, which is an Actor and can create Activities. These Activities are related to producing new posts, restricting access to that content, and notifying other users that they've seen their posts.

The relative path for a person is "/users/{id}/", with child items for a user below that.

### Channel/Service

An Eris *channel* is a Discord text channel that has followed at least one Actor (either an Eris user or a foreign app actor).

It is an ActivityPub Service, which is an Actor and can create Activities. These Activities are related to receiving content.

The relative path for a channel is "/channels/{id}".

### Post/Note

An Eris *post* is an item which can be posted as a Discord [Embed](https://discord.com/safety/using-webhooks-and-embeds#title-4).

It is an ActivityPub Note. It can be Liked and shared (Announce) by anyone, or Updated or Deleted by its original creator.

The relative path for a post is tied to the person who created it, "/users/{user_id}/posts/{post_id}".

### Image and Video

Eris does not host content itself, but allows posts to contain up to one *image* URL and up to one *video* URL.

These are stored as the ActivityPub objects Image and Video, respectively.

Images are located at "/users/{user_id}/posts/{post_id}/attachments/images/{image_id}", videos at "/users/{user_id}/posts/{post_id}/attachments/videos/{video_id}".

### Message

A *message* is a Discord message that has either been turned into a post, or which represent a displayed Post. 

These are not ActivityPub objects, but are tracked in the system and updated/deleted in response to appropriate user actions.

They have no publicly-accessible URL. 

## Foreign entities

Eris also records some information about entities owned by other instances or other ActivityPub-compatible services.

### Foreign Actor

Eris will only accept an incoming Activity from a recognized foreign Actor, and where that Activity has been signed by that actor using their foreign key.

The first time Eris sees an Actor's Id/URL, it will fetch that actor's data from the source server, expecting the returned object to have a "publicKey" field. Eris stores that it has seen this Actor and records their public key.

Then, whenever that Actor posts an Activity, it must be signed with their private key, which is verified using the stored public key. If the verification fails, the Activity is rejected as invalid, because it might have been forged by someone impersonating that Actor.

Apart from the public key, basic information about these Actors also needs to be stored so that Eris can keep track of who is following a local user, and who local channels are following.

### Foreign Object

When presented with a recognized Activity (like Create or Announce) with some foreign Object, Eris will do its best to handle it, but makes no guarantees that behaves consistently with the original application. 

For example, Mastondon uses Create Question to [represent a poll](https://docs.joinmastodon.org/spec/activitypub/#Question). But because Eris (and Discord) don't have the concept of a poll, Eris will try to handle the Question as a Note, using the question's "content" as the body of the post. This means Eris users will see the poll question, but won't see its options, or have the ability to vote on it.