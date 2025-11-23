// src/api/middleware/auth_middleware.rs
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use std::future::{ready, Ready};

pub struct Authentication;

impl actix_web::dev::Transform<actix_web::dev::Service, actix_web::ServiceRequest> for Authentication {
    type Response = actix_web::dev::ServiceResponse;
    type Error = Error;
    type Transform = AuthenticationMiddleware;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: actix_web::dev::Service) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware {
    service: actix_web::dev::Service,
}

impl actix_web::dev::Service<actix_web::ServiceRequest> for AuthenticationMiddleware {
    type Response = actix_web::dev::ServiceResponse;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: actix_web::ServiceRequest) -> Self::Future {
        // Skip authentication for public endpoints
        let path = req.path();
        if path.starts_with("/health") 
            || path.starts_with("/swagger-ui") 
            || path.starts_with("/api-docs")
            || (path.starts_with("/api/v1/users") && req.method() == "POST")
        {
            let fut = self.service.call(req);
            return Box::pin(async move {
                fut.await
            });
        }

        // Extract and validate bearer token for protected endpoints
        let bearer_auth = BearerAuth::extract(&req);
        let fut = self.service.call(req);

        Box::pin(async move {
            match bearer_auth.await {
                Ok(credentials) => {
                    // TODO: Validate JWT token against Keycloak
                    // For now, we'll just log the token and allow access
                    println!("Bearer token: {}", credentials.token());
                    
                    // Add user info to request extensions for use in handlers
                    // req.extensions_mut().insert(UserInfo { sub: "user_id".to_string() });
                    
                    fut.await
                }
                Err(_) => {
                    let config = Config::default();
                    Err(AuthenticationError::from(config).into())
                }
            }
        })
    }
}