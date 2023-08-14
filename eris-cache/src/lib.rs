#![warn(missing_docs)]
//! eris-cache provides [tower::Service]s that abstract over exact cache
//! implementation. It currently supports two different backends: [moka]
//! and [redis]

/// A cache implementation using [moka], an in-memory concurrent hashmap.
pub mod moka;


use std::fmt::{Debug, Display};

use serde::Serialize;
use thiserror::Error;
use tower::Service;

/// A serializable wrapper over a serializable key. This is necessary
/// so that [serde] will serialize the name of the struct into the binary
/// representation. 
#[derive(Debug, Serialize)]
pub(crate) struct CacheKey<K: Serialize> {
    key: K,
}

impl<K: Serialize> From<K> for CacheKey<K> {
    fn from(key: K) -> Self {
        Self {
            key,
        }
    }
}

/// The error that might be returned by a service that has been wrapped in a
/// cache-aside layer.
#[derive(Debug, Error)]
pub enum CacheServiceError<E, S, Req>
where S: Service<Req>, E: Debug + Display, S::Error: Debug + Display
{
    /// The request's key, or its response, could not be serialized
    #[error("Serialization error: {0}")]
    SerializeError(rmp_serde::encode::Error),
    /// The cache failed to produce a response
    #[error("Error executing cache command: {0}")]
    CacheError(E),
    /// The response could not be deserialized
    #[error("Deserialization error: {0}")]
    DeserializeError(rmp_serde::decode::Error),
    /// The inner service was queried and failed to respond
    #[error("{0}")]
    InnerError(S::Error),
}

/// Trait that indicates a query is cacheable, and what key
/// should be used to cache it.
pub trait CacheableQuery {
    /// The type of the key. Must be serializable.
    type Key: Serialize;

    /// Returns the key for this request.
    fn cache_key(&self) -> Self::Key;
}