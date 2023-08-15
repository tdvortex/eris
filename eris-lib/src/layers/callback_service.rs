use std::fmt::{Debug, Display};

use futures_util::future::ready;
use futures_util::FutureExt;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::oneshot::Sender as OneShotSender;
use tower::{service_fn, Service, ServiceExt};

/// The error that might be received from executing a request with a callback.
#[derive(Debug, Error)]
pub enum CallbackServiceError<Req, E: Debug + Display> {
    /// The request was not attempted, returned to sender
    #[error("The service is offline, request not attempted")]
    NotSent(Req),
    /// The request was sent, but the callback channel was closed with
    /// no response
    #[error("The service did not send a response")]
    NoResponse,
    /// The request was attempted but the service returned an error
    #[error("{0}")]
    InnerError(E),
}

async fn callback_service_loop<S, Req>(
    mut service: S,
    mut receiver: UnboundedReceiver<(Req, OneShotSender<Result<S::Response, S::Error>>)>,
) where
    S: Service<Req>,
    S::Error: Debug + Display,
{
    while let Some((request, callback_tx)) = receiver.recv().await {
        let ready_service = match service.ready().await {
            Ok(ready_service) => ready_service,
            Err(e) => {
                if let Err(Err(e)) = callback_tx.send(Err(e)) {
                    tracing::error!("Callback channel closed early, error not delivered: {e}");
                }
                continue;
            }
        };

        if let Err(_) = callback_tx.send(ready_service.call(request).await) {
            tracing::error!("Callback channel closed early, service response not delivered");
        }
    }
}

/// A layer_fn which 
pub fn callback_layer_fn<S, Req>(
    service: S,
) -> impl Service<Req, Response = S::Response, Error = CallbackServiceError<Req, S::Error>>
where
    S: Service<Req> + Send + 'static,
    S::Error: Debug + Display + Send,
    Req: Send + 'static,
    S::Future: Send,
    S::Response: Debug + Send,
{
    let (queue_tx, queue_rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(callback_service_loop(service, queue_rx));

    service_fn(move |request| {
        let (callback_tx, callback_rx) = tokio::sync::oneshot::channel();
        match queue_tx.send((request, callback_tx)) {
            Ok(_) => callback_rx
                .map(|result_result| match result_result {
                    Ok(Ok(response)) => Ok(response),
                    Ok(Err(e)) => Err(CallbackServiceError::InnerError(e)),
                    Err(_recv_error) => Err(CallbackServiceError::NoResponse),
                })
                .left_future(),
            Err(SendError((req, _))) => {
                ready(Err(CallbackServiceError::NotSent(req))).right_future()
            }
        }
    })
}
