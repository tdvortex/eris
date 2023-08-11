use std::{
    fmt::{Debug, Display},
    time::Duration,
};
use tokio::sync::oneshot::{channel as oneshot_channel, Sender as OneShotSender};
use tokio::{
    sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::timeout,
};
use tower::{Service, ServiceExt};

/// An error when trying to process a request through a background service.
#[derive(Debug)]
pub enum BackgroundServiceError<Req, E>
where
    Req: Debug,
    E: Debug + Display,
{
    /// Service closed channel before request was sent
    RequestNotSent(Req),
    /// Service closed channel before responding
    ResponseNotReceived,
    /// Service did not respond within a specified duration
    ResponseTimedOut(Duration),
    /// Service returned an error
    ServiceError(E),
}

impl<Req, E> Display for BackgroundServiceError<Req, E>
where
    Req: Debug,
    E: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundServiceError::RequestNotSent(_) => {
                write!(f, "Service closed channel before request was sent")
            }
            BackgroundServiceError::ResponseNotReceived => {
                write!(f, "Service closed channel before responding")
            }
            BackgroundServiceError::ResponseTimedOut(d) => {
                write!(f, "Service did not respond in {} millis", d.as_millis())
            }
            BackgroundServiceError::ServiceError(e) => {
                write!(f, "Service returned an error: {}", e)
            }
        }
    }
}

/// A handle to a background service.
#[derive(Clone)]
pub struct BackgroundServiceHandle<Req, Resp, E>(
    UnboundedSender<BackgroundServiceRequest<Req, Resp, E>>,
);

impl<Req, Resp, E> BackgroundServiceHandle<Req, Resp, E>
where
    Req: Debug,
    E: Debug + Display,
    Resp: Debug,
{
    /// Invoke the background service with unlimited timeout and await the
    /// response
    pub async fn call(&self, request: Req) -> Result<Resp, BackgroundServiceError<Req, E>> {
        let (callback_tx, callback_rx) = oneshot_channel();
        let backgroud_service_request = BackgroundServiceRequest {
            request,
            callback_tx: Some(callback_tx),
            timeout_dur: None,
        };
        self.0.send(backgroud_service_request).map_err(
            |SendError(backgroud_service_request)| {
                BackgroundServiceError::RequestNotSent(backgroud_service_request.request)
            },
        )?;
        match callback_rx.await {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(BackgroundServiceError::ServiceError(e)),
            Err(_) => Err(BackgroundServiceError::ResponseNotReceived),
        }
    }

    /// Invoke the background service and await a response, but timeout
    /// if it takes too long
    pub async fn call_with_timeout(
        &self,
        request: Req,
        duration: Duration,
    ) -> Result<Resp, BackgroundServiceError<Req, E>> {
        let (callback_tx, callback_rx) = oneshot_channel();
        let backgroud_service_request = BackgroundServiceRequest {
            request,
            callback_tx: Some(callback_tx),
            timeout_dur: Some(duration),
        };
        self.0.send(backgroud_service_request).map_err(
            |SendError(backgroud_service_request)| {
                BackgroundServiceError::RequestNotSent(backgroud_service_request.request)
            },
        )?;
        match callback_rx.await {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(BackgroundServiceError::ServiceError(e)),
            Err(_) => Err(BackgroundServiceError::ResponseNotReceived),
        }
    }

    /// Invoke the background service and don't wait for a response.
    /// Errors will show up in logs but may not be delivered.
    pub fn fire_and_forget(&self, request: Req) -> Result<(), SendError<Req>> {
        let background_service_request = BackgroundServiceRequest {
            request,
            callback_tx: None,
            timeout_dur: None,
        };
        self.0.send(background_service_request).map_err(
            |SendError(background_service_request)| SendError(background_service_request.request),
        )?;
        Ok(())
    }
}

struct BackgroundServiceRequest<Req, Resp, E> {
    request: Req,
    callback_tx: Option<OneShotSender<Result<Resp, E>>>,
    timeout_dur: Option<Duration>,
}

/// Execute with callback and no timeout. Expect the result to be delivered.
async fn execute_background_service<S, Req>(
    service: &mut S,
    request: Req,
) -> Result<S::Response, S::Error>
where
    S: Service<Req>,
{
    service.ready().await?.call(request).await
}

async fn background_service_loop<S, Req>(
    mut service: S,
    mut mpsc_rx: UnboundedReceiver<BackgroundServiceRequest<Req, S::Response, S::Error>>,
) where
    S: Service<Req>,
    S::Error: Display,
{
    while let Some(BackgroundServiceRequest {
        request,
        callback_tx,
        timeout_dur,
    }) = mpsc_rx.recv().await
    {
        let callback_response = if let Some(duration) = timeout_dur {
            if let Ok(result_response) =
                timeout(duration, execute_background_service(&mut service, request)).await
            {
                result_response
            } else {
                tracing::debug!("Background service timed out before producing a response");
                continue;
            }
        } else {
            execute_background_service(&mut service, request).await
        };

        if let Some(callback_tx) = callback_tx {
            if let Err(result_response) = callback_tx.send(callback_response) {
                match result_response {
                    Ok(_) => {
                        tracing::debug!("Background service response callback not delivered");
                    }
                    Err(_) => {
                        tracing::warn!("Background service error callback not delivered");
                    }
                }
            }
        }
    }
}

/// Moves a service into a background thread and returns a handle to that thread.
/// This serves a few purposes:
/// First, this allows the service to be invoked through &self
/// instead of &mut self.
/// Second, the background service handle is Clone even if the
/// underlying service is not Clone.
/// Third, the type signature of the handle does not include the opaque
/// name of the service, allowing the handle to be embedded in a type
/// even if the service was instantiated from a closure with service_fn.
pub fn background_service<S, Req>(service: S) -> BackgroundServiceHandle<Req, S::Response, S::Error>
where
    S: Service<Req> + Send + 'static,
    Req: Send + 'static,
    S::Future: Send,
    S::Error: Display + Send,
    S::Response: Send,
{
    let (mpsc_tx, mpsc_rx) = unbounded_channel();
    let handle = BackgroundServiceHandle(mpsc_tx);
    tokio::spawn(background_service_loop(service, mpsc_rx));
    handle
}
