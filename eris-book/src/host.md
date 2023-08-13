# Running an instance

Most users won't need to run their own Eris instance.

But there are a few benefits to doing so:

1. Choose your own domain name, in addition to WebFinger handle

Every Eris user can choose their own WebFinger handle, but the part after the second @ symbol is the domain of your instance. Running your own instance means you get to choose both!

2. Control your own content moderation, not limited by someone else

There is no one-size-fits-all answer to moderation. Becoming an admin of your own social network means you get to choose the level of content moderation that works for you. 

3. Easier to live within Discord ratelimiting

Discord ratelimits each application/bot to [50 new posts per second](https://discord.com/developers/docs/topics/rate-limits), across all channels, plus an additional limit of 10,000 failed requests every 10 minutes (about 17 per second). To avoid violating this, Eris will throttle the rate that new posts show up. This likely won't be a problem for a small private Eris instance, but for a very large public instances with hundreds of channels, messages might be delayed by a few seconds to account for excess traffic on the network.

This section goes over the key step you'll need to take to launch your own instance.