use std::ops::{Deref, DerefMut};

use futures_util::future::{ready, Ready};
use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender};
use tower::{Service, ServiceExt};

/// An in-memory background task queue service, wrapping a
/// [tokio::sync::mpsc::unbounded_channel] send hook.
/// This service is Clone, so multiple services can insert into
/// this queue at once.
/// This service is fire-and-forget; it provides no mechanism
/// to recover from errors on the other end. It can only
/// detect if it is attempting to send to a closed channel.
#[derive(Debug, Clone)]
pub struct InMemoryQueueService<T>(UnboundedSender<T>);

impl<T> Deref for InMemoryQueueService<T> {
    type Target = UnboundedSender<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Service<T> for InMemoryQueueService<T> {
    type Response = ();

    type Error = SendError<T>;

    type Future = Ready<Result<(), SendError<T>>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        ready(self.send(req))
    }
}

/// The receiving end of the in-memory queue. There can only be
/// one such subscription, so it is not Clone.
#[derive(Debug)]
pub struct InMemoryQueueSubscription<T>(UnboundedReceiver<T>);

impl<T> Deref for InMemoryQueueSubscription<T> {
    type Target = UnboundedReceiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for InMemoryQueueSubscription<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Create an in-memory queue service and a subscription handle to it.
pub fn in_memory_queue<T>() -> (InMemoryQueueService<T>, InMemoryQueueSubscription<T>) {
    let (tx, rx) = unbounded_channel();
    (InMemoryQueueService(tx), InMemoryQueueSubscription(rx))
}

/// Start a service in the background which responds to events in the in-memory
/// queue
pub async fn subscribe_to_queue<S, T>(
    mut service: S,
    mut subscription: InMemoryQueueSubscription<T>,
) where
    S: Service<T, Response = ()> + Send + 'static,
    S::Error: std::fmt::Display + Send,
    S::Future: Send,
    T: Send + 'static,
{
    tokio::spawn(async move {
        while let Some(t) = subscription.recv().await {
            match service.ready().await {
                Ok(service) => {
                    if let Err(e) = service.call(t).await {
                        tracing::error!("Queue subscription service returned an error: {e}");
                    }
                }
                Err(e) => {
                    tracing::error!("Queue subscription service failed to ready: {e}");
                }
            }
        }
    });
}
