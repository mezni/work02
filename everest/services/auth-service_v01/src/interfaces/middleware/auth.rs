use crate::core::errors::AppError;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub keycloak_id: String,
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
        if !auth_str.starts_with("Bearer ") {
            return ready(Err(AppError::Unauthorized(
                "Invalid Authorization format".to_string(),
            )));
        }

        let token = &auth_str[7..];

        // In production, you should validate the JWT token properly
        // For now, we'll decode it without verification (NOT SECURE)
        let token_data = match decode_token(token) {
            Ok(data) => data,
            Err(e) => return ready(Err(e)),
        };

        ready(Ok(token_data))
    }
}

fn decode_token(token: &str) -> Result<AuthenticatedUser, AppError> {
    // This is a simplified version. In production, you should:
    // 1. Validate JWT signature using Keycloak's public key
    // 2. Check token expiration
    // 3. Validate issuer and audience

    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

    #[derive(Debug, Deserialize)]
    struct Claims {
        sub: String,
        email: Option<String>,
        preferred_username: Option<String>,
        realm_access: Option<RealmAccess>,
        network_id: Option<String>,
        station_id: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct RealmAccess {
        roles: Vec<String>,
    }

    // In production, use the actual public key from Keycloak
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation(); // NEVER DO THIS IN PRODUCTION

    let token_data = decode::<Claims>(token, &key, &validation)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    let claims = token_data.claims;

    // Extract role
    let role = claims
        .realm_access
        .and_then(|ra| {
            ra.roles
                .into_iter()
                .find(|r| ["user", "admin", "partner", "operator"].contains(&r.as_str()))
        })
        .unwrap_or_else(|| "user".to_string());

    Ok(AuthenticatedUser {
        user_id: String::new(), // This should be fetched from database based on keycloak_id
        keycloak_id: claims.sub,
        email: claims.email.unwrap_or_default(),
        username: claims.preferred_username.unwrap_or_default(),
        role,
        network_id: claims.network_id,
        station_id: claims.station_id,
    })
}