use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use crate::application::services::auth_service::AuthService;
use std::sync::Arc;

pub struct RbacMiddleware {
    auth_service: Arc<AuthService>,
    required_roles: Vec<String>,
}

impl RbacMiddleware {
    pub fn new(auth_service: Arc<AuthService>, required_roles: Vec<&str>) -> Self {
        Self {
            auth_service,
            required_roles: required_roles.into_iter().map(String::from).collect(),
        }
    }

    pub async fn validator(
        &self,
        mut req: ServiceRequest, // Make req mutable
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        let token = credentials.token();

        // Validate token
        let claims = match self.auth_service.validate_token(token) {
            Ok(claims) => claims,
            Err(_) => {
                let config = req.app_data::<Config>().cloned().unwrap_or_default();
                return Err((AuthenticationError::from(config).into(), req));
            }
        };

        // Add user claims to request extensions
        req.extensions_mut().insert(claims);
        
        Ok(req)
    }
}

// Helper function to create middleware for specific roles
pub fn require_roles(auth_service: Arc<AuthService>, roles: Vec<&str>) -> RbacMiddleware {
    RbacMiddleware::new(auth_service, roles)
}