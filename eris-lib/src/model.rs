/// The base entity and actor in the system.
pub mod application;

/// The Block Activity.
pub mod block;

/// A Discord channel, which is also an Actor
/// in ActivityPub so that it can follow others.
pub mod channel;

/// The Follow Activity.
pub mod follow;

/// Any Actor from an external source.
pub mod foreign_actor;

/// An Image object, mostly used as an attachment.
pub mod image;

/// The Like activity.
pub mod like;

/// A Discord message. This is **not** ActivityPub
/// but does need an internal data representation.\
pub mod message;

/// A post on the network.
pub mod post;

/// A tombstone marker for a deleted entity.
pub mod tombstone;

/// A human app user. One-to-one with a Discord user.
pub mod user;

/// A Video object, mostly used as an attachment.
pub mod video;