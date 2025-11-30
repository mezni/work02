use tracing::info;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{forward_ready, Service, Transform};
use std::future::{ready, Ready, Future};
use std::pin::Pin;

pub struct RequestLogging;

impl<S, B> Transform<S, ServiceRequest> for RequestLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggingMiddleware { service }))
    }
}

pub struct RequestLoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        
        info!("Incoming request: {} {}", method, path);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            info!("Response status: {}", res.status());
            Ok(res)
        })
    }
}
