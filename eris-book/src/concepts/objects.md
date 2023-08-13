# Objects

Eris uses a few conceptual entities, most of which are ActivityPub Objects.

## Local entities

These are the "things" that Eris creates, hosts, and distributes to the Fediverse.

### Instance/Application

An Eris *instance* is one server, with one web domain, tied to a specific Discord bot.

It is an ActivityPub Appliciation-type Object. It is an Actor, meaning it can create Activities. These are admin-level actions which relate to the operation of the instance itself, including user management and channel management. 

The relative path for an instance is just the root of that instance's domain, "/".

### User/Person

An Eris *user* is a Discord user that has additionally signed up with an Eris instance using the "/join" slash command. 

They are an ActivityPub Person, which is an Actor and can create Activities. These Activities are related to producing new posts, restricting access to that content, and reacting to viewed content.

The relative path for a person is "/users/{id}/", with child items for a user below that.

### Channel/Service

An Eris *channel* is a Discord text channel that has been registered with the instance.

It is an ActivityPub Service, which is an Actor and can create Activities. These Activities are related to receiving and filtering content.

The relative path for a channel is "/channels/{guild_id}/{channel_id}".

### Post/Note

An Eris *post* is an item which can be posted as a Discord [Embed](https://discord.com/safety/using-webhooks-and-embeds#title-4).

It is an ActivityPub Note. It can be Liked and shared (Announced) by anyone, Updated by its original author, or Deleted by its original creator or an instance admin.

The relative path for a post is tied to the person who created it, "/users/{user_id}/posts/{post_id}".

### Image and Video

Eris does not host content itself, but allows posts to contain up to one *image* URL and up to one *video* URL.

These links are stored as the ActivityPub objects Image and Video, respectively.

Images are located at "/users/{user_id}/posts/{post_id}/attachments/images/{image_id}", videos at "/users/{user_id}/posts/{post_id}/attachments/videos/{video_id}".

### Message

A *message* is a Discord message which represents a displayed post. Because a post may be shared across many channels, there may be many different messages that all reference the same post.

These are not ActivityPub objects, but are tracked in the system and updated/deleted whenever their parent post is updated/deleted.

They have no publicly-accessible URL. 

## Foreign entities

Eris also records some information about entities owned by other instances or other ActivityPub-compatible services.

### Foreign Actor

Eris will only accept an incoming Activity from a recognized foreign Actor, and where that Activity has been signed by that actor using a verified public key.

The first time Eris sees an Actor's Id/URL, it will fetch that actor's data from the source server, expecting the returned object to have a "publicKey" field. Eris stores that it has seen this Actor and records their public key.

Then, whenever that Actor posts an Activity, it must be signed with their private key, which is verified using the stored public key. If the verification fails, the Activity is rejected as invalid, because it might have been forged by someone impersonating that Actor.

Apart from the public key, basic information about these Actors also needs to be stored so that Eris can keep track of who is following a local user, and who local channels are following.

### Foreign Object

When presented with a recognized Activity (like Create or Announce) with some foreign Object, Eris will do its best to handle it, but makes no guarantees that behaves consistently with the original application. 

For example, Mastondon uses the Create Question Activity to [represent a poll](https://docs.joinmastodon.org/spec/activitypub/#Question). But because Eris (and Discord) don't have the concept of a poll, Eris will try to handle the Question as a Note, using the question's "content" as the body of the post. This means Eris users will see the poll question, but won't see its options, or have the ability to vote on it. 