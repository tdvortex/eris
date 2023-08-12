# Core features

What does it mean to bridge Discord with the Fediverse? 

## Comparison with other apps

### Feed-based social networks

On most social networks, a user will publish some kind of content, and will receive a *feed* or timeline of content posted or re-shared by other users. This means that users are forming relationships with other users, usually with a "follows" or "subscribes" relation. 

Most of the differences are in what kinds of content are posted; one website might focus on short-form text uploads, another might be a video sharing service, another might be audio uploads.

Posts on most feed-based social networks are public by default; a Mastodon status can usually be seen by anyone who has a link to that post. 

Users can usually show their approval of content by "liking" it, and optionally "sharing" (also called "resharing", "reblogging", "boosting", or other terms), which is handled as if the user had posted it themselves, but attributes it to the correct original user.

There is usually also a "direct message" system or its or equivalent for private user-to-user communication.

### Discord

Discord, meanwhile, is most like a traditional chat service. Chat rooms are organized into what Discord calls "servers" or sometimes "guilds", with each server/guild having many channels. Users can also directly message each other if they are friends (which essentially just approves the messaging channel).

Users can join a server and (depending on permissions) view and post in these shared chat rooms. On Discord, a message always is private to a specific channel, and access privileges are controlled by the admins of that server/guild.

A larger Discord community server can often be extremely fast-moving, with a channel seeing many users chatting simultaenously in real time. But, because chat messages last forever, a smaller or slower-moving server might behave more like a forum board, with larger posts at a lower frequency.

### Other structures

Of course, other systems are possible. [Lemmy](https://join-lemmy.org), for example, lets users post content to a community, view a community's conetnt, and vote or comment on it.

## Eris

Eris is, essentially, a feed-based social network that uses Discord as its primary point of access. So while users will primarily interact with the network via Discord bot commands, its internal structure is similar to something like Mastodon or Pixelfed.

There are a few unique characterstics that this presents.

### Users vs Channels

A **user** is a Discord user, usually a human (but possibly an external bot). Users post content and can be followed. Users *cannot follow anything*; users do not have an individualized, personal feed to post things to. 

A **channel** is a Discord text channel in a Discord server/guild. Channels display posts, and can follow users. But channels *cannot post anything*, so they cannot be followed. 

So, instead of a user only seeing their own personalized feed (like on [Mastodon](https://joinmastodon.org/)), a user can see many channels, and a channel is shared amongst many users. 

### Posts are public but unlisted

Like feed-based social networks (*a la* Mastodon), when a user makes a message into a post, is is publicly available. It will be directly posted on *all* channels which follow them, even those the user themselves doesn't see and may not even know about. Then, he post might also be re-shared by users in those channels, or shared onto other social networks; anyone who has a link to the post can see it without any authorization required.

This means that any post made using Eris could potentially be seen by thousands of people, including complete strangers, anywhere in the world, even if the source message was posted to a private Discord channel. **Do not post sensitive information onto Eris** (or any other public social network)!

However, while a post is publicly *accessible*, it is not publicly *broadcast*. Eris does not have the concept of a "public feed"; it would be too overwhelming to post every message to every channel! 

If you post a message, it will be shared with your followers (other channels, or users of other applications). But if you don't have any followers, then no one will ever see it (unless you manually share the link some other way).

### Users can Like, Channels can't

Users are the only ones who can actually choose to *like* a post. Channels display posts, but they aren't people, and are incapable of liking things.

Imagine if a controversial post showed up on a channel, and half the users thought it was good and the other half thought it was terrible. It wouldn't make sense to either say that "the channel likes this" or "the channel dislikes this", so this isn't allowed.

### Channels and users can block each other

The only action that both channels *and* users can do is to block one another.

When a user blocks a channel, this means "I do not want my posts to show up in this channel". This applies even for reshares; no one can post your content onto this channel. However, this doesn't retroactively unpost messages that were already shared on the channel. It simply means that channel receives no updates about your posts in the future, as if you went silent and stopped posting.

Users can also block other users. This similarly means "I do not want my posts to be forwarded to this person". However, because Eris users do not personally receive messages, this doesn't actually block them from seeing your posts if it's shard on a channel they have access to. And, it doesn't prevent their messages from showing up on a channel that you can see.

Channels can also block users. This means "I do not want this user's posts to show up". Again, this applies even for reshares, making a channel block like a "super unfollow".

Channels cannot block other channels. This would be meaningless; they can't send messages to each other, so there's nothing to block.

### No direct messages

Most feed-based social networks also offer some kind of direct message system for private communication. 

Eris does not currently offer a user-to-user direct messaging option. 

Because every Eris user is also a Discord user, if an Eris user wants to DM another Eris user, they can just DM them directly. 

For cross-app direct messaging (for example, an Eris user DMing with a Mastodon user) there are difficulties related to Discord's interface, because Eris can only open one DM channel per user no matter how many DM conversations they have concurrently.