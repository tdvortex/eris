use tower::{Service, Layer};

use crate::payloads::DiscordServerAction;

/// A layer which provides a cloned queue Service to another Service, 
/// enabling the consuming Service to send DiscordServerActions into the
/// queue service.
pub struct DiscordServerActionQueueProviderLayer<Q> {
    queue_service: Q,
}

impl<Q> DiscordServerActionQueueProviderLayer<Q> {
    /// Create a new DiscordServerActionQueueProviderLayer
    pub fn new(queue_service: Q) -> Self {
        Self { queue_service }
    }
}

impl<Q, S> Layer<S> for DiscordServerActionQueueProviderLayer<Q> 
    where Q: Clone
{
    type Service = DiscordServerActionQueueService<Q, S>;

    fn layer(&self, inner: S) -> Self::Service {
        DiscordServerActionQueueService {
            queue_service: self.queue_service.clone(),
            inner,
        }
    }
}

/// Wraps a service that requires a DiscordServerAction queue with that queue,
/// reducing its request type from (Q, Request) to just Request.
/// Q will be cloned for every request to S so it should typically be wrapped
/// in an Arc or otherwise be cheaply clonable.
pub struct DiscordServerActionQueueService<Q, S> {
    queue_service: Q,
    inner: S,
}

impl<Q, S, Request> Service<Request> for DiscordServerActionQueueService<Q, S>
where
    Q: Service<DiscordServerAction, Response = ()>,
    Q: Clone,
    S: Service<(Q, Request)>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call((self.queue_service.clone(), req))
    }
}
