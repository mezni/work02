// src/api/middleware/logging_middleware.rs
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware { service }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let start = std::time::Instant::now();

        let fut = self.service.call(req);

        Box::pin(async move {
            let result = fut.await;
            let duration = start.elapsed();
            
            match &result {
                Ok(response) => {
                    log::info!(
                        "{} {} {} - {}ms",
                        method,
                        path,
                        response.status().as_u16(),
                        duration.as_millis()
                    );
                }
                Err(_) => {
                    log::warn!(
                        "{} {} ERROR - {}ms",
                        method,
                        path,
                        duration.as_millis()
                    );
                }
            }
            
            result
        })
    }
}