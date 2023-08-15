use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

use bytes::{BufMut, Bytes, BytesMut};
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use tower::{layer::layer_fn, service_fn, Layer, Service, ServiceExt};

use crate::{CacheKey, CacheServiceError, CacheableQuery};

/// Returns a [tower::Layer] which converts a
pub fn cache_aside_layer<S, Req>(
    moka_cache: Cache<Bytes, Bytes>,
) -> impl Layer<
    S,
    Service = impl Service<
        Vec<Req>,
        Response = Vec<Result<S::Response, CacheServiceError<Infallible, S, Req>>>,
        Error = Infallible,
    > + Clone,
>
where
    S: Service<Req> + Clone,
    Req: CacheableQuery,
    S::Response: Serialize + DeserializeOwned,
    S::Error: Debug + Display,
{
    layer_fn(move |service: S| {
        let moka_cache = moka_cache.clone();
        service_fn(move |veq_request: Vec<Req>| {
            let moka_cache = moka_cache.clone();
            let mut service = service.clone();
            async move {
                let mut responses = Vec::with_capacity(veq_request.len());

                for request in veq_request.into_iter() {
                    let cache_key: Bytes = {
                        let mut writer = BytesMut::with_capacity(128).writer();
                        if let Err(e) = rmp_serde::encode::write(
                            &mut writer,
                            &CacheKey::from(request.cache_key()),
                        ) {
                            responses.push(Err(CacheServiceError::SerializeError(e)));
                            continue;
                        }
                        writer.into_inner().into()
                    };

                    let response = if let Some(response_bytes) = moka_cache.get(&cache_key) {
                        // Cache hit, deserialize it and don't do any extra reads or writes
                        match rmp_serde::from_slice(&response_bytes) {
                            Ok(response) => response,
                            Err(e) => {
                                responses.push(Err(CacheServiceError::DeserializeError(e)));
                                continue;
                            }
                        }
                    } else {
                        // Cache miss, get the value from the inner service
                        // Make sure the service is ready
                        let ready_service = match service.ready().await {
                            Ok(ready_service) => ready_service,
                            Err(e) => {
                                responses.push(Err(CacheServiceError::InnerError(e)));
                                continue;
                            }
                        };

                        // Call the service and see if it succeeds
                        let response = match ready_service.call(request).await {
                            Ok(response) => response,
                            Err(e) => {
                                responses.push(Err(CacheServiceError::InnerError(e)));
                                continue;
                            }
                        };

                        // Serialize response and insert into the cache
                        let response_bytes: Bytes = {
                            let mut writer = BytesMut::with_capacity(128).writer();
                            if let Err(e) = rmp_serde::encode::write(&mut writer, &response) {
                                responses.push(Err(CacheServiceError::SerializeError(e)));
                                continue;
                            }
                            writer.into_inner().into()
                        };
                        moka_cache.insert(cache_key, response_bytes).await;
                        response
                    };

                    responses.push(Ok(response));
                }

                Ok(responses)
            }
        })
    })
}
