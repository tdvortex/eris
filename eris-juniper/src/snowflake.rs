use juniper::GraphQLScalarValue;

/// A wrapper type around a Discord snowflake to make it usable as a GraphQL
/// scalar type.
#[derive(GraphQLScalarValue)]
pub struct Snowflake(String);