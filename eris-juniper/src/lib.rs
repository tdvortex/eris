#![warn(missing_docs)]
//! eris-juniper defines the GraphQL API for Eris.
//! It conforms to the [Cursor Connections](https://graphql.org/learn/global-object-identification/)
//! specification and the [Global Object Identification](https://graphql.org/learn/global-object-identification/)
//! specification.

mod context;
pub use context::MockContext;

/// Edge types, which represent a relationship between two Nodes.
pub mod edges;

/// Interfaces which standardize functionality.
pub mod interfaces;

/// Node types, representing a queryable entity.
pub mod nodes;

/// Scalar types, defining concrete (non-query) data.
pub mod scalars;

mod query;
pub use query::Query;
