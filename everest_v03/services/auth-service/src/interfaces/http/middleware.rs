use crate::core::errors::AppError;
use crate::core::jwt::{extract_bearer_token, verify_token, Claims};
use actix_cors::Cors;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{ready, Ready};

/// CORS middleware configuration
pub fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}

/// Authenticated user extracted from JWT token
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,           // Database user_id (USRxxxx)
    pub keycloak_id: Option<String>,
    pub email: String,
    pub username: String,
    pub role: String,
    pub network_id: Option<String>,
    pub station_id: Option<String>,
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Extract Authorization header
        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h,
            None => {
                return ready(Err(AppError::Unauthorized(
                    "Missing Authorization header".to_string(),
                )))
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return ready(Err(AppError::Unauthorized(
                    "Invalid Authorization header".to_string(),
                )))
            }
        };

        // Extract Bearer token
        let token = match extract_bearer_token(auth_str) {
            Ok(t) => t,
            Err(e) => return ready(Err(e)),
        };

        // Verify and decode token
        let claims = match verify_token(token) {
            Ok(c) => c,
            Err(e) => return ready(Err(e)),
        };

        // Convert claims to AuthenticatedUser
        ready(Ok(AuthenticatedUser {
            user_id: claims.sub,
            keycloak_id: claims.keycloak_id,
            email: claims.email.unwrap_or_default(),
            username: claims.preferred_username.unwrap_or_default(),
            role: claims.get_role(),
            network_id: claims.network_id,
            station_id: claims.station_id,
        }))
    }
}

/// Optional authenticated user (for endpoints that work with or without auth)
pub struct OptionalAuth(pub Option<AuthenticatedUser>);

impl FromRequest for OptionalAuth {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h,
            None => return ready(Ok(OptionalAuth(None))),
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => return ready(Ok(OptionalAuth(None))),
        };

        let token = match extract_bearer_token(auth_str) {
            Ok(t) => t,
            Err(_) => return ready(Ok(OptionalAuth(None))),
        };

        let claims = match verify_token(token) {
            Ok(c) => c,
            Err(_) => return ready(Ok(OptionalAuth(None))),
        };

        let user = AuthenticatedUser {
            user_id: claims.sub.clone(),
            keycloak_id: claims.keycloak_id.clone(),
            email: claims.email.clone().unwrap_or_default(),
            username: claims.preferred_username.clone().unwrap_or_default(),
            role: claims.get_role(),
            network_id: claims.network_id.clone(),
            station_id: claims.station_id.clone(),
        };

        ready(Ok(OptionalAuth(Some(user))))
    }
}

/// Helper to extract geo location from request
pub fn extract_geo_info(req: &HttpRequest) -> Option<crate::domain::audit::GeoLocation> {
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())?;

    // In production, use MaxMind GeoIP2 or similar service
    // For now, just return IP
    Some(crate::domain::audit::GeoLocation::new(ip))
}

/// Helper to extract user agent from request
pub fn extract_user_agent(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bearer_token() {
        let header = "Bearer test_token";
        let token = extract_bearer_token(header).unwrap();
        assert_eq!(token, "test_token");
    }

    #[test]
    fn test_extract_bearer_token_invalid() {
        let header = "Basic credentials";
        assert!(extract_bearer_token(header).is_err());
    }
}