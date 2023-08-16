use juniper::graphql_interface;


#[graphql_interface]
/// A node, representing any individually queryable entity.
pub trait Node {
    /// Returns the node's Base64-encoded [NodeId], which indicates both the
    /// concrete Rust type of the object as well as any unique identifiers
    /// it requires.
    fn id(&self) -> String;
}