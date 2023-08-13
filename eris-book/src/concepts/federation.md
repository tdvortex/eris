# Federation

The "fed" in Fediverse stands for "federated", which is a form of **decentralization** in web architecture.

Eris is a federated social network, that connects Discord (a centralized social network) with other federated social networks.

## Centralized client-server architecture

The majority of the web operates in a **client-server** system, with a high degree of **centralization**.

As the name suggests, in a client-server system, you have some machines which are *servers* and others which are *clients*. 

A server owns some data. Clients can make requests to view or modify that data, but the server can deny those requests for any reason. The server can also choose to destroy data, edit data, hide data, sell data to third parties, charge a fee to access the data, or inject ads in the data they return.

The biggest social networks in the world are all highly centralized. Facebook, Instagram, Twitter/X, Youtube, TikTok, Twitch, Reddit, Tumblr, and Discord are all centralized, with massive server clusters working together to collect and oversee their users' social activity.

This gives these companies massive power over their users' social lives, and with hundreds of millions or even billions of users, that's massive control over the world in general. This makes these single central systems vulnerable to hacking, propaganda, or even just simple mismanagement, and users have no choice but to accept this, or leave the network entirely.

These social networks also often don't play nice together; a video creator who posts short-form content to both Youtube Shorts and TikTok has to upload the same content twice, with different subscribers on each platform. They can't just post on TikTok and then reshare it with their Youtube account.

## Distributed peer-to-peer architecture

At the other extreme, we have **peer-to-peer** systems. In a peer-to-peer system, every machine is an equal participant, a *peer*.

Instead of a central authority making the rules, in a peer-to-peer system the participants all agree to a shared *protocol*, the rules for interacting with each other. If a peer breaks these rules, the other peers will reject their actions, so the only way to participate is to play along.

Data in a peer-to-peer system does not live in one place; instead, copies are **distributed** across the machines in a network. This makes it impossible for any one peer to delete or edit the data. Peer-to-peer file-sharing systems like [Bittorrent](https://www.bittorrent.com/) are very popular for spreading pirated or otherwise illegal content, because there is no one single source to take down.

Peer-to-peer systems using a [blockchain](https://en.wikipedia.org/wiki/Blockchain) protocol do allow for write-only data using a *consensus* mechanism. However, the incentive for reaching consensus is normally tied to a [cryptocurrency](https://en.wikipedia.org/wiki/Cryptocurrency) like Bitcoin or Ether, a speculative investment that users hope to be able to spend or sell later. 

While an immutable, permanent, public history might work for an auditable financial tool like a cryptocurrency, it doesn't work very well for a social network. Social networks need privacy, but the trust-free nature of a blockchain makes privacy virtually impossible. Social networks need moderation, a way to prevent abuse and harassment, but a peer-to-peer network cannot distinguish between justified moderation and hostile censorship. Many users also don't want their social lives tied to a cryptocurrency.

## Decentralized federated architecture

Federation strikes a balance between client-server and peer-to-peer. In a federated system, you still have clients and servers, but the servers all talk to each other using a shared protocol as peers.

This means that a server has control over its own data. It can choose to selectively limit the data it shares with other servers or with its own clients. The servers receiving data can, in turn, choose to interpret that data as they choose, forwarding it on as necessary.

Users are free to choose whichever server they like, or to start their own personal server. If you think a server is under-moderated and full of spam, you can leave and find a new server that better moderates its content. Or if you think a server is censoring too much content, you can leave and find a server with freer speech. 

A centralized system demands that users completely trust the central server. A peer-to-per system operates trustlessly, where everyone is constantly on-guard for hostile peers. A decentralized system allows for provisional trust between client and server that can be revoked.