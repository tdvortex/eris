# Foreign Actor Activities

Eris cannot predict all of the possible ActivityPub actions that other social networks might allow, now or in the future. Eris will attempt to translate received actions into local terms as much as possible, but some imprecision is to be expected.

For example, Mastodon has (limited) support for HTML in status bodies. Discord only supports Markdown. Eris will not attempt to parse and translate Mastodon HTML into Markdown, so the posted format may be somewhat confusing.

Eris will only respond to Activities that are attributed to Actors with a public key that can be verified by either internal records or a GET quey to the host server. If an Actor does not have a public key, or states an unverifiable public key, the Activity will be denied and ignored, as it cannot be confirmed legitimate.
## Create Object

If an Eric channel receives a Create Object message, it will interpret that as a foreign Actor attempting to create a post. 

If that Actor has been blocked (by either the channel or the instance), it will do nothing.

If the Actor has not been blocked, then Eris will check the type and available fields of the Object and try to interpret it into a postable format.

Discord messages support [Embeds](https://discord.com/developers/docs/resources/channel#embed-object) with up to 4096s character of body text, up to one Image (with a URL), up to one Video (with a URL), up to one Author (with a name, and optionally a URL), an original source URL, and a timestamp. Eris will attempt to read these fields from the Created Object; if successful, it will post the message to the channel.

## Update Object

When an Eris instance receives an Update Object Activity from a non-blocked Actor, it will look for any posted messages which link to the Updated Object, and update them to match the new Object provided along with the Update. This inference is the same as for the original Create Object message.

## Delete Object

When an Eris instance receives a Delete Object Activity from a non-blocked Actor, it will look for any posted messages which link to the Deleted Object, and delete them.

## Like post

If a foreign Actor Likes a post that resides on the instance, and the Actor has not been blocked by either the instance or the post's original author, Eris will update the post's likes to include the foreign Actor. 

## Undo Like post

If a foreign Actor that previously Liked a post that resides on the instance, and later sends an Undo Like Activity, and the Actor has not been blocked by either the instance or the post's original author, Eris will remove the actor from the post's likes.

## Announce Object

If a foreign Actor Announces an Object to a channel, and the Actor has not been blocked by either the instance or the channel, and the channel has not already sent a message for this Object, then the channel will attempt to post the Object as an Embed in the same manner as the Create Object method. 

If the Object being Announced is a post owned by the instance, then Eris will also add the Actor's ID to the post's "shares" collection.

## Undo Announce post

If a foreign Actor Undoes an Announce for an post, and the Actor has not been blocked by either the instance or the channel, then their ID will be removed from the post's "shares" collection.

However, Undoing an Announce does **not** delete messages from a channel that have already been shared.

## Block user

According to the ActivityPub spec, Block Activities should not be sent to the instances of the blocked Actors. [Mastodon does this](https://docs.joinmastodon.org/spec/activitypub/#Block), however, to indicate that that Actor's Activities should not be sent to the blocking instance.

Eris honors this request by not sending any user Activities to an Actor that has blocked them. 

## Undo Block User

If Eris receives an Undo Block activity from a foreign Actor on a local user, it will undo the Block action and resume sending that user's content as normal.

## Block channel

According to the ActivityPub spec, Block Activities should not be sent to the instances of the blocked Actors. [Mastodon does this](https://docs.joinmastodon.org/spec/activitypub/#Block), however, to indicate that that Actor's Activities should not be sent to the blocking instance.

Eris honors this request by not displaying any Objects with the blocking Actor in the "attributedTo" field in the channel. This prevents both Create activities (posting) and Announce activities (resharing).

## Undo Block channel

If Eris receives an Undo Block activity from a foreign Actor on a local user, it will undo the Block action and allow the channel to show that user's posted content.

## Accept Follow

If a foreign Actor Accepts a Follow Activity originating from the channel, the channel will add the Actor to its following Collection.

## Reject Follow

If a foreign Actor Rejects a Follow Activity originating from the channel, the channel will not add the Actor to its following Collection.