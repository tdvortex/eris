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

/// Utilities that can be used for scheduling service execution that are not
/// themselves Services.
pub mod scheduling;

/// [serde] compatible payloads for queue services.
pub mod payloads;