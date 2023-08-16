use juniper::graphql_interface;
use url::Url;

use super::{ActivityPubObject, Node};

#[graphql_interface]
/// An ActivityPub Actor, capable of performing Activities.
/// As per the spec, must have an inbox URL and an outbox URL.
/// Eris additionally requires that all Actors have a public key to
/// verify signed Activities.
pub trait Actor: ActivityPubObject {
    /// The URL of the actor's inbox.
    fn inbox_url(&self) -> Url;
    /// The URL of the actor's outbox.
    fn outbox_url(&self) -> Url;
    /// The actor's public key.
    fn public_key_pem(&self) -> String;
}

impl ActivityPubObject for ActorValue {
    #[doc = " The URL for this object."]
    fn activitypub_id(&self) -> Url {
        todo!()
    }
}

impl Node for ActorValue {
    #[doc = " Returns the node\\'s Base64-encoded [NodeId], which indicates both the"]
    #[doc = " concrete Rust type of the object as well as any unique identifiers"]
    #[doc = " it requires."]
    fn id(&self) -> String {
        todo!()
    }
}