use actix_web::{dev::Service, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::task::{Context, Poll};
use tracing::info;

pub struct AuditMiddleware<S> {
    service: S,
}

impl<S, B> Service<actix_web::dev::ServiceRequest> for AuditMiddleware<S>
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
        // Here you would extract audit information from the request
        // and potentially log it after the response is generated
        
        let fut = self.service.call(req);

        Box::pin(async move {
            let response = fut.await?;
            
            // Extract audit data from request extensions and response
            // Log the audit event using the Auditor
            
            Ok(response)
        })
    }
}

pub struct Audit;

impl Audit {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for Audit
where
    S: Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuditMiddleware<S>;
    type Future = futures_util::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(AuditMiddleware { service })
    }
}