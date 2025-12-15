use crate::core::errors::{AppError, AppResult};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,                          // Database user_id (USRxxxx)
    pub keycloak_id: Option<String>,          // Keycloak user ID
    pub email: Option<String>,
    pub preferred_username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<HashMap<String, ResourceAccess>>,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: Option<serde_json::Value>,
    
    // Custom attributes from Keycloak
    pub network_id: Option<String>,
    pub station_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceAccess {
    pub roles: Vec<String>,
}

impl Claims {
    /// Get the primary role from realm roles
    pub fn get_role(&self) -> String {
        self.realm_access
            .as_ref()
            .and_then(|ra| {
                ra.roles
                    .iter()
                    .find(|r| ["user", "admin", "partner", "operator"].contains(&r.as_str()))
            })
            .cloned()
            .unwrap_or_else(|| "user".to_string())
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.realm_access
            .as_ref()
            .map(|ra| ra.roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

/// Verify JWT token (simplified - in production use Keycloak's public key)
pub fn verify_token(token: &str) -> AppResult<Claims> {
    // In production, you should:
    // 1. Fetch Keycloak's JWKS endpoint
    // 2. Get the public key matching the token's kid
    // 3. Use that key to verify the signature
    
    // For now, we'll do basic decoding without verification (NOT SECURE FOR PRODUCTION)
    let header = decode_header(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token header: {}", e)))?;

    // Create validation
    let mut validation = Validation::new(Algorithm::RS256);
    validation.insecure_disable_signature_validation(); // NEVER DO THIS IN PRODUCTION
    validation.validate_exp = true;

    // Decode token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&[]), // This should be Keycloak's public key
        &validation,
    )
    .map_err(|e| AppError::Unauthorized(format!("Token verification failed: {}", e)))?;

    Ok(token_data.claims)
}

/// Extract Bearer token from Authorization header
pub fn extract_bearer_token(auth_header: &str) -> AppResult<&str> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized(
            "Invalid authorization format. Expected: Bearer <token>".to_string(),
        ));
    }

    Ok(&auth_header[7..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bearer_token() {
        let header = "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.test.test";
        let token = extract_bearer_token(header).unwrap();
        assert_eq!(token, "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.test.test");
    }

    #[test]
    fn test_extract_bearer_token_invalid() {
        let header = "Basic dXNlcjpwYXNzd29yZA==";
        let result = extract_bearer_token(header);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_get_role() {
        let claims = Claims {
            sub: "123".to_string(),
            email: None,
            preferred_username: None,
            given_name: None,
            family_name: None,
            realm_access: Some(RealmAccess {
                roles: vec!["admin".to_string(), "other".to_string()],
            }),
            resource_access: None,
            exp: 0,
            iat: 0,
            iss: "".to_string(),
            aud: None,
            network_id: None,
            station_id: None,
        };

        assert_eq!(claims.get_role(), "admin");
        assert!(claims.is_admin());
        assert!(claims.has_role("admin"));
    }
}