use std::{sync::Arc, marker::PhantomData};

use tower::{Layer, Service};

/// A layer which provides a clonable state parameter of type T
/// to a service processing requests of type (T, R), reducing it
/// to a service which takes requests of type R alone.
pub struct ClonedStateProviderLayer<T, R>
where T: Clone,
{
    state: T,
    _inner_request: PhantomData<R>,
}

impl<T, R> ClonedStateProviderLayer<T, R> 
where T: Clone
{
    /// Creates a new ClonedStateProviderLayer with a clonable T
    pub fn new(state: T) -> Self {
        Self { state, _inner_request: PhantomData }
    }
}

impl<T, R> ClonedStateProviderLayer<Arc<T>, R> {
    /// Creates a new ClonedStateProviderLayer with a T that may or may not be
    /// Clone, by wrapping it in an Arc first. This makes it relatively cheap
    /// to clone, at the cost of only implementing Deref<T> instead of Deref<Mut>
    /// or Into<T>.
    pub fn with_arc(state: T) -> Self {
        Self { state: Arc::new(state), _inner_request: PhantomData }
    }
}

impl<T, S, R> Layer<S> for ClonedStateProviderLayer<T, R> 
where T: Clone, S: Service<(T, R)>
{
    type Service = CloneStateProviderService<T, S, R>;

    fn layer(&self, inner: S) -> Self::Service {
        CloneStateProviderService {
            state: self.state.clone(),
            inner,
            _inner_request: PhantomData,
        }
    }
}

/// A service which wraps a Service<(T, R)>, making it a
/// Service<R> by cloning T for every request.
pub struct CloneStateProviderService<T, S, R> 
where T: Clone, S: Service<(T, R)>
{
    state: T,
    inner: S,
    _inner_request: PhantomData<R>,
}

impl<T, S, R> Service<R> for CloneStateProviderService<T, S, R> 
where
    S: Service<(T, R)>,
    T: Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: R) -> Self::Future {
        self.inner.call((self.state.clone(), req))
    }
}