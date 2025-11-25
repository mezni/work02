use actix_web::{dev::Service, web, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::task::{Context, Poll};
use tracing::{info, warn, Span};
use uuid::Uuid;

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<actix_web::dev::ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let request_id = Uuid::new_v4().to_string();

        // Add request ID to extensions for use in handlers
        req.extensions_mut().insert(request_id.clone());

        // Set up tracing span
        let span = tracing::info_span!(
            "request",
            method = %method,
            path = %path,
            request_id = %request_id
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let response = fut.await?;
            let duration = start.elapsed();
            let status = response.status();

            if status.is_server_error() {
                warn!(
                    "Request completed with error - method: {}, path: {}, status: {}, duration: {:?}",
                    method, path, status, duration
                );
            } else {
                info!(
                    "Request completed - method: {}, path: {}, status: {}, duration: {:?}",
                    method, path, status, duration
                );
            }

            Ok(response)
        })
    }
}

pub struct Logging;

impl Logging {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for Logging
where
    S: Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = futures_util::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(LoggingMiddleware { service })
    }
}