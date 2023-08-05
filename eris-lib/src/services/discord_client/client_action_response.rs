use twilight_model::channel::Message;

/// A response to an action taken by the Discord client service.
pub enum ClientActionResponse {
    /// A CreateMessage request was successful. Requires storing the message id
    /// and other important fields in the database in case we need to edit or
    /// delete it later.
    MessageCreated(Message),
}