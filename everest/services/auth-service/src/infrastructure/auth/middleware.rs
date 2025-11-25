use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::{info, warn, error};

use crate::domain::enums::UserRole;
use crate::infrastructure::auth::jwt::Claims;
use crate::infrastructure::errors::InfrastructureError;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn validate_token(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        let token = credentials.token();
        
        // Get JWT service from app data
        let jwt_service = req
            .app_data::<crate::infrastructure::auth::JwtService>()
            .expect("JwtService not configured in app data");

        match jwt_service.validate_token(token) {
            Ok(claims) => {
                // Clone claims before moving it
                let user_id = claims.sub.clone();
                
                // Add claims to request extensions for use in handlers
                req.extensions_mut().insert(claims);
                info!("Token validated successfully for user: {}", user_id);
                Ok(req)
            }
            Err(e) => {
                warn!("Token validation failed: {}", e);
                Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
            }
        }
    }
}

pub struct RoleGuard {
    required_roles: Vec<UserRole>,
}

impl RoleGuard {
    pub fn new(required_roles: Vec<UserRole>) -> Self {
        Self { required_roles }
    }

    pub fn check(&self, user_role: &UserRole) -> bool {
        self.required_roles.contains(user_role)
    }

    pub fn admin() -> Self {
        Self::new(vec![UserRole::Admin])
    }

    pub fn company_management() -> Self {
        Self::new(vec![UserRole::Admin, UserRole::Partner, UserRole::Operator])
    }

    pub fn user_management() -> Self {
        Self::new(vec![UserRole::Admin, UserRole::Partner, UserRole::Operator])
    }

    pub fn authenticated() -> Self {
        Self::new(vec![
            UserRole::Admin,
            UserRole::Partner, 
            UserRole::Operator,
            UserRole::User,
            UserRole::Guest,
        ])
    }
}

pub fn extract_user_role_from_request(req: &ServiceRequest) -> Result<UserRole, InfrastructureError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| {
            InfrastructureError::Authorization("No claims found in request".to_string())
        })?;

    UserRole::from_str(&claims.role)
        .ok_or_else(|| {
            InfrastructureError::Authorization(format!("Invalid role in token: {}", claims.role))
        })
}

pub fn extract_user_id_from_request(req: &ServiceRequest) -> Result<String, InfrastructureError> {
    let extensions = req.extensions();
    let claims = extensions
        .get::<Claims>()
        .ok_or_else(|| {
            InfrastructureError::Authorization("No claims found in request".to_string())
        })?;

    Ok(claims.sub.clone())
}

pub fn extract_company_id_from_request(req: &ServiceRequest) -> Option<String> {
    let extensions = req.extensions();
    extensions
        .get::<Claims>()
        .and_then(|claims| claims.company_id.clone())
}