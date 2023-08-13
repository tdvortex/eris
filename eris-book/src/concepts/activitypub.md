# ActivityPub

Eris federates its content using the [ActivityPub](https://activitypub.rocks/) protocol.

## ActivityPub is kind of like email

A federated service most people already familiar with is email. Email uses the [Simple Mail Transfer Protocol](https://en.wikipedia.org/wiki/Simple_Mail_Transfer_Protocol) (SMTP).

When you get an email address, it is always "at" a specific domain. If a person has an account "username@website.com", that means that their account "username" is hosted by "website.com". 

From the perspective of the protocol, "username@website.com" is a completely different person to "username@other-website.net", even if both accounts are owned by the same real-world person. This can be useful; many people have a "professional" email where they want to receive business solitications and send work items, and a "personal" email they use for emailing friends and family.

SMTP supports one operation: *user sends email to \[targets\]*. Here's how that works:

1. A user writes the text of the message, and pushes it to their *outbox*.
2. Their email server looks at the destination server of that email; if it's a different server, it forwards it on.
3. When an email server receives an email addressed to one of its users, it puts that email in that user's *inbox*.
4. The receiving user asks their server for emails in their inbox, and is provided a list of every new email they haven't yet seen.

Email servers can, at their choice, choose *not* to deliver messages to other servers or clients. And there's a very good reason to do so--spam. An email server that is bombarded with spam might block the sender accounts, or even entire spam servers, and refuse to deliver their emails to its clients. 

Of course, if spam filtering is done poorly, it can end up hiding emails that users actually want to see. That's the tradeoff; anyone you trust to moderate can also censor. But there are lots of email providers, and it's not too hard to set up your own SMTP server if you like, so no one is forced to live with a bad email provider. 

## Activities and Objects

ActivityPub starts off very similar to email, with users having an account "at" a specific *instance* of a particular network. 

Officially, an ActivityPub user account is framed as a plain URL, like "https://mastodon.social/users/Gargron". But, Mastodon and most other networks additionally support the [WebFinger](https://webfinger.net/) protocol, which also allows formats like "@Gargron@mastodon.social". These mean the same thing; the user is Gargron, the host is mastodon.social.

Where ActivityPub is different from SMTP is that it supports a much broader range of actions. The more general pattern can be described as: *Actor* does *Activity* to *Object* and notifies anyone in the *\[to/cc/bcc\]* fields and/or an *audience*.

Breaking that down:
* An **Actor** is an agent in the system which is able to create Activities. All actors must have an Inbox and an Outbox, just like email.
* An **Activity** is some kind of a verb, something which changes the overall state of the system. Common activites include Create, Follow, Like, and Announce (which is what ActivityPub calls resharing).
* An **Object** is any conceptual "thing" that exists on a specific instance. Every Object has a unique URl, which says which server owns it and where to find it again. Actors and Activities are both Objects, but there are also non-Actor, non-Activity Objects, representing the actual content on a network, such as Notes, Images, and Videos.
* ActivityPub supports direct, targeted delivery using familiar email terms: "to", "cc", and "bcc". 
* ActivityPub also supports distributed delivery with the *audience* field on an Activity. This field points to a special kind of Object, a **Collection** of Actors, saying "this Activity is targeted to everyone in this group".

Here's what a few basic actions look like:

* **Posting**: a Person sends a Create Note to an audience of their followers.
* **Liking**: a Person sends a Like Note to the Person that published it.
* **Following**: a Person sends a Follow Person to that Person, who responds by sending an Accept Follow back.
* **Sharing**: a Person sends an Announce Note to all of their followers and to the original Note's poster, who records them in the Object's "shares" Collection.
* **Deleting**: a Person sends a Delete Note to the audience of a Note, and to everyone in its Shares. The receiving servers then replace that Object with a special Object called a Tombstone to say "there used to be something here, but it's gone now".

When a user creates an Activity, their origin server will validate it, and if it's good, forward it on to anyone in to/cc/bcc and to anyone in audience. 


Just like email, a server can choose not to send certain Activities, and can choose not to accept delivery of certain Activities that it decides are invalid. This can be used for spam filtering and moderation, but it can also be used to define what a particular social network is capable of. For example, Lemmy's concept of a "Community" has no direct analogue in Mastodon, so if Lemmy sent a "Create Community" Activity to Mastodon, the Mastodon server would just ignore it because that doesn't mean anything in its view of the world. 