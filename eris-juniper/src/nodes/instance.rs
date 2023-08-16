use juniper::GraphQLObject;
use url::Url;

/// The GraphQL object representing the instance.
#[derive(GraphQLObject)]
pub struct Instance {
    domain: Url,
}