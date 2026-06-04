use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

const X_REQUEST_ID: &str = "x-request-id";

/// Injects an `x-request-id` header into every request (and mirrors it in
/// the response) so traces can be correlated end-to-end.
pub struct RequestId;

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestIdMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = Rc::clone(&self.service);

        let request_id = req
            .headers()
            .get(X_REQUEST_ID)
            .and_then(|v| v.to_str().ok())
            .map_or_else(|| Uuid::new_v4().to_string(), str::to_owned);

        // Ensure the header is normalised on the request.
        req.headers_mut().insert(
            actix_web::http::header::HeaderName::from_static(X_REQUEST_ID),
            actix_web::http::header::HeaderValue::from_str(&request_id)
                .unwrap_or_else(|_| actix_web::http::header::HeaderValue::from_static("unknown")),
        );

        Box::pin(async move {
            let mut res = svc.call(req).await?;
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static(X_REQUEST_ID),
                actix_web::http::header::HeaderValue::from_str(&request_id).unwrap_or_else(|_| {
                    actix_web::http::header::HeaderValue::from_static("unknown")
                }),
            );
            Ok(res)
        })
    }
}
