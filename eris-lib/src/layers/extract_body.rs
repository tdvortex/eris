use http::Request;
use tower::{Service, ServiceExt};

/// A layer_fn which calls Request::into_body and passes the body on to the 
/// inner service.
pub fn extract_body_layer_fn<S, T>(service: S) -> impl Service<Request<T>, Response = S::Response, Error = S::Error, Future = S::Future> 
where S:Service<T>
{
    service.map_request(|request: Request<T>| request.into_body())    
}