use juniper::graphql_interface;
use url::Url;

use super::Node;

#[graphql_interface]
/// An ActivityPub Object, with no other guarantees. May be a local Actor,
/// a foreign Actor, a locally-created Object, an Activity, or any other
/// item which ActivityPub recognizes.
pub trait ActivityPubObject: Node {
    /// The URL for this object.
    fn activitypub_id(&self) -> Url;
}

impl Node for ActivityPubObjectValue {
    #[doc = " Returns the node\\'s Base64-encoded [NodeId], which indicates both the"]
    #[doc = " concrete Rust type of the object as well as any unique identifiers"]
    #[doc = " it requires."]
    fn id(&self) -> String {
        todo!()
    }
}