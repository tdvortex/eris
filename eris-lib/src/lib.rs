#![warn(missing_docs)]
//! eris-lib defines the logic of the Eris social network as a number of
//! interdependent [`tower::Service`]s and [`tower::Layer`]s, as well as
//! functions which can be loaded into a binary and executed to deploy
//! to Discord.

/// Commands to deploy services and Discord application commands
pub mod deploy;

/// [`tower::Layer`]s that can be used to assemble other [`tower::Service`]s.
pub mod layers;

/// [`tower::Service]s that can be used to implement binaries.
pub mod services;

/// [serde] compatible payloads for queue services.
pub mod payloads;