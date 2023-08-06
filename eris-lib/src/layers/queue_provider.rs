use std::{fmt::{Debug, Display}, marker::PhantomData};

use futures_util::Future;
use pin_project::pin_project;
use thiserror::Error;
use tower::{Service, Layer};

#[derive(Debug, Error)]
pub enum QueueProviderLayerError<Q, I> 
    where Q: Debug + Display,
    I: Debug + Display,
{
    #[error("Error with the queueing service: {0}")]
    QueueError(Q),
    #[error("Error with the inner service: {0}")]
    InnerError(I),
}


/// A layer which provides a cloned queue Service to another Service, 
/// enabling the consuming Service to send messages into the
/// queue service.
pub struct QueueProviderLayer<Q, T> 
    where Q: Service<T> + Clone
{
    queue_service: Q,
    _message: PhantomData<T>,
}

impl<Q, T> QueueProviderLayer<Q, T> 
    where Q: Service<T> + Clone
{
    /// Create a new DiscordServerActionQueueProviderLayer
    pub fn new(queue_service: Q) -> Self {
        Self { queue_service, _message: PhantomData }
    }
}

impl<Q, T, S> Layer<S> for QueueProviderLayer<Q, T> 
    where Q: Service<T> + Clone
{
    type Service = QueueProviderService<Q, T, S>;

    fn layer(&self, inner: S) -> Self::Service {
        QueueProviderService {
            queue_service: self.queue_service.clone(),
            inner,
            _message: PhantomData,
        }
    }
}

/// Wraps a service that requires a queue to send Ts into with that queue,
/// reducing its request type from (Q, Request) to just Request.
/// Q will be cloned for every request to S so it should typically be wrapped
/// in an Arc or otherwise be cheaply clonable.
pub struct QueueProviderService<Q, T, S> 
    where Q: Service<T>
{
    queue_service: Q,
    inner: S,
    _message: PhantomData<T>,
}

#[pin_project]
pub struct QueueProviderFuture<F, Q> {
    #[pin]
    future: F,
    _queue_error: PhantomData<Q>,
}

impl<F, Q> QueueProviderFuture<F, Q> {
    fn new(future: F) -> Self {
        Self {
            future,
            _queue_error: PhantomData
        }
    }
}

impl<F, T, Q, I> Future for QueueProviderFuture<F, Q> 
    where F: Future<Output = Result<T, I>>,
    Q: Debug + Display,
    I: Debug + Display,
{
    type Output = Result<T, QueueProviderLayerError<Q, I>>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.project().future.poll(cx) {
            std::task::Poll::Ready(Ok(t)) => std::task::Poll::Ready(Ok(t)),
            std::task::Poll::Ready(Err(inner_error)) => std::task::Poll::Ready(Err(QueueProviderLayerError::InnerError(inner_error))),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}


impl<Q, S, T, Request> Service<Request> for QueueProviderService<Q, T, S>
where
    Q: Service<T, Response = ()>,
    Q::Error: Debug + Display,
    Q: Clone,
    S: Service<(Q, Request)>,
    S::Error: Debug + Display,

{
    type Response = S::Response;
    type Error = QueueProviderLayerError<Q::Error, S::Error>;
    type Future = QueueProviderFuture<S::Future, Q::Error>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let queue_ready = self.queue_service.poll_ready(cx);
        let inner_ready = self.inner.poll_ready(cx);

        match (queue_ready, inner_ready) {
            // If both services are ready and error-free, this service is
            // ready and error-free
            (std::task::Poll::Ready(Ok(_)), std::task::Poll::Ready(Ok(_))) => {
                std::task::Poll::Ready(Ok(()))
            }
            // If there are no errors and at least one service is pending,
            // this service is pending
            (std::task::Poll::Pending, std::task::Poll::Ready(Ok(_)))
            | (std::task::Poll::Ready(Ok(_)), std::task::Poll::Pending)
            | (std::task::Poll::Pending, std::task::Poll::Pending) => std::task::Poll::Pending,
            // If the inner service is in an error state, this service is in
            // an error state
            (_, std::task::Poll::Ready(Err(e))) => std::task::Poll::Ready(Err(QueueProviderLayerError::InnerError(e))),
            (std::task::Poll::Ready(Err(e)), _) => std::task::Poll::Ready(Err(QueueProviderLayerError::QueueError(e))),
        }
    }

    fn call(&mut self, req: Request) -> Self::Future {
        QueueProviderFuture::new(self.inner.call((self.queue_service.clone(), req)))
    }
}
