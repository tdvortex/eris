use http::Request;
use tower::{layer::layer_fn, Layer, Service, ServiceExt};

use hyper::Body as HyperBody;
use lambda_http::Body as LambdaBody;

/// Returns a compatibility layer that converts a [`tower::Service`] from taking a
/// [`http::Request`] with a [`hyper::Body`] payload to one that takes a
/// [`lambda_http::Body`] payload.
pub fn lambda_compatibility_layer<
    S: Service<Request<HyperBody>> + ServiceExt<Request<HyperBody>>,
>() -> impl Layer<S> {
    layer_fn(|service: S| {
        service.map_request(|lambda_request: Request<LambdaBody>| {
            let (parts, body) = lambda_request.into_parts();

            let body = match body {
                LambdaBody::Empty => HyperBody::empty(),
                LambdaBody::Text(s) => HyperBody::from(s),
                LambdaBody::Binary(v) => HyperBody::from(v),
            };

            Request::from_parts(parts, body)
        })
    })
}