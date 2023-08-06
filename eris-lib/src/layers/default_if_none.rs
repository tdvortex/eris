use futures_util::{future::ready, FutureExt};
use tower::{service_fn, Service};

/// A layer_fn which turns a Service that takes a Request and returns a
/// Default-able Response into a service which takes an Option and
/// returns the default if the request is None.
pub fn default_if_none_layer_fn<S, R>(
    mut service: S,
) -> impl Service<Option<R>, Response = S::Response, Error = S::Error>
where
    S: Service<R>,
    S::Response: Default,
{
    service_fn(move |request: Option<R>| match request {
        Some(request) => service.call(request).left_future(),
        None => ready(Ok(S::Response::default())).right_future(),
    })
}
