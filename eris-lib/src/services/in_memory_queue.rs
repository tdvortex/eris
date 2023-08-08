use futures_util::future::{Ready, ready};
use tokio::sync::mpsc::{UnboundedSender, error::SendError};
use tower::Service;

/// An in-memory background task queue service, wrapping a
/// [tokio::sync::mpsc::unbounded_channel] send hook.
/// This service is Clone, so multiple services can insert into
/// this queue at once. 
#[derive(Debug, Clone)]
pub struct InMemoryQueueService<T>(UnboundedSender<T>);

impl<T> From<UnboundedSender<T>> for InMemoryQueueService<T> {
    fn from(value: UnboundedSender<T>) -> Self {
        Self(value)
    }
}

impl<T> Service<T> for InMemoryQueueService<T> {
    type Response = ();

    type Error = SendError<T>;

    type Future = Ready<Result<(), SendError<T>>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        ready(self.0.send(req))
    }
}