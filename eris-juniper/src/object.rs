use juniper::graphql_interface;
use url::Url;

#[graphql_interface]
pub trait Object {
    fn activitypub_id(&self) -> Url;
}