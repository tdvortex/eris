use std::fmt::{Debug, Display};

use http::Request;
use hyper::body::Bytes;
use thiserror::Error;
use tower::{service_fn, Service, ServiceExt};

/// An error which might be returned by the the body-to-bytes service.
#[derive(Debug, Error)]
pub enum BodyToBytesServiceError<B, I>
where
    B: http_body::Body,
    B::Error: Debug + Display,
    I: Debug + Display,
{
    /// An error when trying to convert the request body into a Bytes object.
    #[error("Error attempting to convert into bytes: {0}")]
    ToBytesError(B::Error),
    /// An error returned by the inner service (after a successful Bytes conversion).
    #[error("Error in the inner service: {0}")]
    InnerError(I),
}

async fn body_to_bytes<B>(request: Request<B>) -> Result<Request<Bytes>, B::Error>
where
    B: http_body::Body,
{
    let (parts, body) = request.into_parts();
    let bytes = hyper::body::to_bytes(body).await?;
    Ok(Request::from_parts(parts, bytes))
}

/// A function which can be passed to [tower::layer::layer_fn] to map a service which takes
/// Request<Bytes> into one that takes Request<B: Body>.
pub fn body_to_bytes_layer_fn<B, S>(
    mut service: S,
) -> impl Service<Request<B>, Response = S::Response, Error = BodyToBytesServiceError<B, S::Error>> + Clone
where
    B: http_body::Body,
    S: Service<Request<Bytes>> + Clone,
    B::Error: Debug + Display,
    S::Error: Debug + Display,
{
    service_fn(body_to_bytes)
    .map_err(BodyToBytesServiceError::ToBytesError)
    .then(|result_request_bytes| async move {
        match result_request_bytes {
            Ok(request_bytes) => match service.ready().await {
                Ok(ready_service) => match ready_service.call(request_bytes).await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(BodyToBytesServiceError::InnerError(e)),
                },
                Err(e) => Err(BodyToBytesServiceError::InnerError(e)),
            },
            Err(e) => Err(e),
        }
    })
}