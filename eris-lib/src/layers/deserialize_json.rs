use std::fmt::{Debug, Display};

use futures_util::future::ready;
use http::Request;
use hyper::body::Bytes;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tower::{service_fn, Service, ServiceExt};

/// The error that might occur when attempting to deserialize a Bytes payload
/// from a Request or using that payload.
#[derive(Debug, Error)]
pub enum JsonDeserializationServiceError<E: Debug + Display> {
    /// Deserialization error
    #[error("Could not deserialize Json: {0}")]
    JsonDeserialization(serde_json::Error),
    /// Error pass-through from inner service
    #[error("Error in inner service: {0}")]
    InnerError(E),
}

fn deserialize_json<T>(request: Request<Bytes>) -> Result<Request<T>, serde_json::Error>
where
    T: DeserializeOwned,
{
    let (parts, body) = request.into_parts();
    let payload = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, payload))
}

/// A function which can be used with [tower::layer::layer_fn] to convert a 
/// Service taking Request<T> as input to one that takes Request<Bytes>.
pub async fn deserialize_json_layer_fn<S, T>(
    mut service: S,
) -> impl Service<
    Request<Bytes>,
    Response = S::Response,
    Error = JsonDeserializationServiceError<S::Error>,
> + Clone
where
    S: Service<Request<T>>,
    S: Clone,
    T: DeserializeOwned,
    S::Error: Debug + Display,
{
    service_fn(|request| {
        ready(
            deserialize_json(request).map_err(JsonDeserializationServiceError::JsonDeserialization),
        )
    })
    .then(|result_request| async move {
        match result_request {
            Ok(request) => match service.ready().await {
                Ok(ready_service) => match ready_service.call(request).await {
                    Ok(response) => Ok(response),
                    Err(e) => Err(JsonDeserializationServiceError::InnerError(e)),
                },
                Err(e) => Err(JsonDeserializationServiceError::InnerError(e)),
            },
            Err(e) => Err(e),
        }
    })
}
