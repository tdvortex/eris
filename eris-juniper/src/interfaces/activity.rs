use juniper::graphql_interface;
use url::Url;

use super::{ActivityPubObject, Node, actor::ActorValue, ActivityPubObjectValue};

#[graphql_interface]
/// An ActivityPub Activity, representing a state-affecting action taken
/// by some Actor. Usually has an object, but may not for intransitive activities.
pub trait Activity: ActivityPubObject {
    /// The Actor performing the Activity.
    fn actor(&self) -> ActorValue;
    /// The object of the Activity.
    fn object(&self) -> Option<ActivityPubObjectValue>;
}

impl ActivityPubObject for ActivityValue {
    #[doc = " The URL for this object."]
    fn activitypub_id(&self) -> Url {
        todo!()
    }
}

impl Node for ActivityValue {
    #[doc = " Returns the node\\'s Base64-encoded [NodeId], which indicates both the"]
    #[doc = " concrete Rust type of the object as well as any unique identifiers"]
    #[doc = " it requires."]
    fn id(&self) -> String {
        todo!()
    }
}