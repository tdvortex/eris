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

// pub struct LambdaCompatabilityLayer;

// impl<S> Layer<S> for LambdaCompatabilityLayer {
//     type Service = LambdaCompatabilityService<S>;

//     fn layer(&self, inner: S) -> Self::Service {
//         LambdaCompatabilityService {
//             inner,
//         }
//     }
// }

// /// A middleware [`tower::Service`] which converts a [`lambda_http::Body`] into a
// /// [`hyper::Body`]
// pub struct LambdaCompatabilityService<S> {
//     inner: S,
// }

// impl<S> Service<Request<LambdaBody>> for LambdaCompatabilityService<S>
//     where S: Service<Request<HyperBody>, Response = (StatusCode, JsonValue)>
// {
//     type Response = (StatusCode, JsonValue);

//     type Error = S::Error;

//     type Future = S::Future;

//     fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, req: Request<LambdaBody>) -> Self::Future {
//         let (parts, body) = req.into_parts();

//         let body = match body {
//             LambdaBody::Empty => todo!(),
//             LambdaBody::Text(s) => HyperBody::from(s),
//             LambdaBody::Binary(v) => HyperBody::from(v),
//         };

//         let req = Request::from_parts(parts, body);

//         self.inner.call(req)

//     }
// }
