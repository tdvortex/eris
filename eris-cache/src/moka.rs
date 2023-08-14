use std::{convert::Infallible, fmt::{Debug, Display}};

use bytes::{BufMut, Bytes, BytesMut};
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use tower::{Service, service_fn, Layer, layer::layer_fn, ServiceExt};

use crate::{CacheServiceError, CacheableQuery, CacheKey};

/// Returns a [tower::Layer] which converts a 
pub fn cache_aside_layer<S, Req>(moka_cache: Cache<Bytes, Bytes>) -> impl Layer<S, Service = impl Service<Vec<Req>, Response = Vec<S::Response>, Error = CacheServiceError<Infallible, S, Req>> + Clone>
where S: Service<Req> + Clone,
    Req: CacheableQuery,
    S::Response: Serialize + DeserializeOwned, 
    S::Error: Debug + Display
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
                            rmp_serde::encode::write(&mut writer, &CacheKey::from(request.cache_key())).map_err(CacheServiceError::SerializeError)?;
                            writer.into_inner().into()
                        };

                        let response = if let Some(response_bytes) = moka_cache.get(&cache_key) {
                            // Cache hit, deserialize it and don't do any extra reads or writes
                            rmp_serde::from_slice(&response_bytes).map_err(CacheServiceError::DeserializeError)?
                        } else {
                            // Cache miss, get the value from the inner service
                            let response = service.ready().await.map_err(CacheServiceError::InnerError)?.call(request).await.map_err(CacheServiceError::InnerError)?;
                            
                            // Serialize response and insert into the cache
                            let response_bytes: Bytes = {
                                let mut writer = BytesMut::with_capacity(128).writer();
                                rmp_serde::encode::write(&mut writer, &response).map_err(CacheServiceError::SerializeError)?;
                                writer.into_inner().into()
                            };
                            moka_cache.insert(cache_key, response_bytes).await;
                            response
                        };

                        responses.push(response)
                    }

                    Ok(responses)
            }})
        })
    }