// src/interfaces/middleware/auth_middleware.rs
use crate::application::services::auth_service::AuthService;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use std::sync::Arc;

pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn validator(
        &self,
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        let token = credentials.token();

        match self.auth_service.validate_token(token) {
            Ok(claims) => {
                // Add user claims to request extensions
                req.extensions_mut().insert(claims);
                Ok(req)
            }
            Err(_e) => {
                let config = req.app_data::<Config>().cloned().unwrap_or_default();
                Err((AuthenticationError::from(config).into(), req))
            }
        }
    }
}
