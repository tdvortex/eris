use std::sync::Arc;

use tower::{Layer, Service};
use twilight_model::id::{marker::ApplicationMarker, Id};

/// The shared state required to make requests to Discord using
/// [`twilight_http::Client`].
#[derive(Debug)]
pub struct TwilightClientState {
    /// The client itself
    pub twilight_client: twilight_http::Client,
    /// The application Id. Used for interaction endpoints.
    pub application_id: Id<ApplicationMarker>,
}

/// A layer which provides [TwilightClientState] to a [tower::Service]
/// that takes ([Arc]<[TwilightClientState]>, R) as its Request, converting it
/// into a service that only takes R as its Request.
pub struct TwilightClientProviderLayer {
    state: Arc<TwilightClientState>,
}

impl TwilightClientProviderLayer {
    /// Create a new [TwilightClientProviderLayer]
    pub fn new(
        twilight_client: twilight_http::Client,
        application_id: Id<ApplicationMarker>,
    ) -> Self {
        Self {
            state: Arc::new(TwilightClientState {
                twilight_client,
                application_id,
            }),
        }
    }
}

impl<S> Layer<S> for TwilightClientProviderLayer {
    type Service = TwilightClientService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TwilightClientService {
            state: self.state.clone(),
            inner,
        }
    }
}

/// A [tower::Service] that has been provided with a [TwilightClientState]
/// (wrapped in an [Arc] for easy cloning.
pub struct TwilightClientService<S> {
    state: Arc<TwilightClientState>,
    inner: S,
}

impl<S, Request> Service<Request> for TwilightClientService<S>
where
    S: Service<(Arc<TwilightClientState>, Request)>,
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
        self.inner.call((self.state.clone(), req))
    }
}
