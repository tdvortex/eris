use futures_util::{FutureExt, future::{ready, Either, Ready}};
use tower::{Layer, Service};

/// A layer which turns a Service that takes a Request and returns a 
/// Default-able Response into a service which takes an Option and
/// returns the default if the request is None.
pub struct DefaultIfNoneLayer;

impl<S> Layer<S> for DefaultIfNoneLayer {
    type Service = DefaultIfNoneService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DefaultIfNoneService { inner }
    }
}

/// A service that returns a default response if passed None as
/// the request, or runs an inner service on the Some value.
pub struct DefaultIfNoneService<S> {
    inner: S,
}

impl<S, Request> Service<Option<Request>> for DefaultIfNoneService<S>
where
    S: Service<Request>,
    S::Response: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Either<S::Future, Ready<Result<S::Response, S::Error>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Option<Request>) -> Self::Future {
        if let Some(req) = req {
            self.inner.call(req).left_future()
        } else {
            ready(Ok(S::Response::default())).right_future()
        }
    }
}
