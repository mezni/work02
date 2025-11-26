use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::future::{ready, Ready};
use crate::infrastructure::keycloak::KeycloakClient;

pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Get Keycloak client from app data
    let keycloak_client = req.app_data::<actix_web::web::Data<KeycloakClient>>()
        .ok_or_else(|| {
            (actix_web::error::ErrorInternalServerError("Keycloak client not configured"), req)
        })?;

    let token = credentials.token();
    
    match keycloak_client.verify_token(token).await {
        Ok(user_info) => {
            let user = AuthenticatedUser {
                id: user_info.sub,
                username: user_info.username,
                email: user_info.email,
                roles: user_info.roles,
            };
            req.extensions_mut().insert(user);
            Ok(req)
        }
        Err(_) => {
            Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
        }
    }
}

// Helper function to check if user has required role
pub fn has_required_role(user: &AuthenticatedUser, required_role: &str) -> bool {
    user.roles.iter().any(|role| role == required_role)
}

// Helper function to extract user from request
pub fn get_authenticated_user(req: &actix_web::HttpRequest) -> Option<&AuthenticatedUser> {
    req.extensions().get::<AuthenticatedUser>()
}