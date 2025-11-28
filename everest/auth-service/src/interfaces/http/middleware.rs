use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    Error, HttpMessage,
};
use std::{
    future::{ready, Ready, Future},
    pin::Pin,
    rc::Rc,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::infrastructure::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub role: String,
    pub organisation_name: Option<String>,
    pub station_name: Option<String>,
    pub preferred_username: String,
}

pub struct AuthGuard;

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthGuardMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        
        Box::pin(async move {
            // Skip auth for public routes
            if req.path().starts_with("/api/v1/auth/login") || 
               req.path().starts_with("/api/v1/auth/register") ||
               req.path().starts_with("/health") ||
               req.path().starts_with("/swagger-ui") ||
               req.path().starts_with("/api-doc") {
                return service.call(req).await;
            }

            // Extract JWT token from Authorization header
            let token = req.headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .and_then(|header| {
                    if header.starts_with("Bearer ") {
                        Some(&header[7..])
                    } else {
                        None
                    }
                });

            if let Some(token) = token {
                // In a real implementation, you would get the config from app data
                // For now, we'll use a placeholder validation
                if validate_token(token).is_ok() {
                    // Token is valid, proceed with the request
                    return service.call(req).await;
                }
            }

            // Token is invalid or missing
            let (http_req, _) = req.into_parts();
            let response = actix_web::HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "Unauthorized"}))
                .into_body();
            let http_res = actix_web::HttpResponse::Unauthorized().set_body(response);
            Ok(ServiceResponse::new(http_req, http_res))
        })
    }
}

fn validate_token(_token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    // This is a simplified validation
    // In a real implementation, you would:
    // 1. Get the JWT secret from configuration
    // 2. Validate the token signature and expiration
    // 3. Return the claims if valid
    
    // For now, we'll accept any non-empty token as valid in this stub
    if _token.is_empty() {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken
        ));
    }
    
    // Stub claims - in real implementation, decode the token
    Ok(JwtClaims {
        sub: "user_id".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
        role: "user".to_string(),
        organisation_name: None,
        station_name: None,
        preferred_username: "user".to_string(),
    })
}
