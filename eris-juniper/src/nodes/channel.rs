use juniper::GraphQLObject;
use url::Url;

use crate::{Snowflake, object::ActivityPubObject};

#[derive(GraphQLObject)]
pub struct Channel {
    guild_id: Snowflake,
    channel_id: Snowflake,
}

impl ActivityPubObject for Channel {
    fn activitypub_id(&self) -> Url {
        todo!()
    }
}