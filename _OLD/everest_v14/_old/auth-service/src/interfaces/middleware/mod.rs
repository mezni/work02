pub mod auth;
pub mod logging;

// src/interfaces/middleware/auth.rs
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::domain::value_objects::Role;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub keycloak_id: String,
    pub role: Role,
    pub organisation_name: Option<String>,
}

impl actix_web::FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            req.extensions()
                .get::<AuthenticatedUser>()
                .cloned()
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))
        })
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    
    // In production, decode and validate JWT token from Keycloak
    // For now, we'll use a simplified approach
    // let claims = decode_token(token, public_key)?;
    
    // Mock authenticated user for demonstration
    let auth_user = AuthenticatedUser {
        user_id: Uuid::new_v4(),
        keycloak_id: "mock_keycloak_id".to_string(),
        role: Role::Admin, // Extract from token claims in production
        organisation_name: None,
    };

    req.extensions_mut().insert(auth_user);
    Ok(req)
}
