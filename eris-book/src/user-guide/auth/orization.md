# Authorization

If you are taking actions through an HTTP client or via the REST or GraphQL APIs, there are four levels of authorization possible: Public, User, Admin, and Not Available.

## Public

The following actions can be taken without authentication:

* View general instance information, including the Ids of its admins
* View a user's WebFinger handle, and public key
* View a list of a user's posts, followers, or likes
* View a specific post by a user
* View an image or video URL attached to a post
* View the Actors a channel is following

## User

The following actions can be taken only by an authenticated user:

* Join an instance
* Change your WebFinger username
* Delete your account on an instance. **This cannot be undone**.
* Disable/enable auto-accept follows (enabled by default)
* Create, update, or delete a post
* Like a post, or remove a like from a post
* Share an Object as a post, or stop sharing an Object
* Block another Actor by their URL, or unblock them

## Admin

The following actions can only be taken by an authenticated instance admin:

* View a list of all users on the instance
* View a list of all channels on the instance
* View a list of all post messages in a channel (does not show non-post messages)
* View a list of all foreign actors the instance knows about, with their IDs and public keys
* Delete a user. **This cannot be undone**.
* Delete a channel's Actor. This does not delete the Discord channel itself. **This cannot be undone**.
* Delete a post made by a user (across all channels).
* Ban an Actor (local user, channel, or foreign actor)
* Unban an Actor
* Update instance settings, such as disabling open enrollment, or changing the admin list

## Not available

For security reasons, the following actions cannot be executed from an HTTP client, even by an admin:

* View an actor's private key
* View who an instance, user, or channel is blocking
* Create a new channel
* Create an account for a different user
* Delete an instance
* Delete a post message without deleting the underlying Post object

These actions are still possible for someone with direct access to an instance's database(s).