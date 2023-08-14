use juniper::graphql_object;

use crate::{MockContext, Instance};

/// The root Query object.
pub struct Query;

#[graphql_object(Context = MockContext)]
impl Query {
    fn instance(_context: &MockContext) -> Instance {
        todo!()
    }
}