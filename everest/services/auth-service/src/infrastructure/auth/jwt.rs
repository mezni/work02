use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::infrastructure::config::Settings;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
    pub jti: String, // JWT ID
    pub username: String,
    pub email: String,
    pub role: String,
    pub company_id: Option<String>,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(settings: &Settings) -> Self {
        let secret = settings.auth.jwt_secret.as_bytes();
        
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            expiration_seconds: settings.auth.jwt_expiration_seconds,
        }
    }

    pub fn generate_token(
        &self,
        user_id: &str,
        username: &str,
        email: &str,
        role: &str,
        company_id: Option<&str>,
    ) -> Result<String, InfrastructureError> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(self.expiration_seconds as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            company_id: company_id.map(|id| id.to_string()),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| {
                error!("Failed to generate JWT token: {}", e);
                InfrastructureError::Jwt(e)
            })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, InfrastructureError> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| {
                warn!("JWT token validation failed: {}", e);
                InfrastructureError::Jwt(e)
            })?;

        Ok(token_data.claims)
    }

    pub fn refresh_token(&self, token: &str) -> Result<String, InfrastructureError> {
        let claims = self.validate_token(token)?;
        
        // Generate new token with same claims but new expiration
        self.generate_token(
            &claims.sub,
            &claims.username,
            &claims.email,
            &claims.role,
            claims.company_id.as_deref(),
        )
    }

    pub fn get_remaining_time(&self, token: &str) -> Result<Duration, InfrastructureError> {
        let claims = self.validate_token(token)?;
        let now = Utc::now().timestamp() as usize;
        
        if claims.exp <= now {
            return Ok(Duration::seconds(0));
        }
        
        let remaining_seconds = claims.exp - now;
        Ok(Duration::seconds(remaining_seconds as i64))
    }

    pub fn is_token_expired(&self, token: &str) -> bool {
        match self.get_remaining_time(token) {
            Ok(duration) => duration.num_seconds() <= 0,
            Err(_) => true, // If we can't validate, consider it expired
        }
    }

    pub fn extract_user_id(&self, token: &str) -> Result<String, InfrastructureError> {
        let claims = self.validate_token(token)?;
        Ok(claims.sub)
    }

    pub fn extract_role(&self, token: &str) -> Result<String, InfrastructureError> {
        let claims = self.validate_token(token)?;
        Ok(claims.role)
    }

    pub fn extract_company_id(&self, token: &str) -> Result<Option<String>, InfrastructureError> {
        let claims = self.validate_token(token)?;
        Ok(claims.company_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::config::{Settings, AuthSettings, DatabaseSettings, KeycloakSettings, ServerSettings, CacheSettings, AuditSettings};

    fn create_test_settings() -> Settings {
        Settings {
            database: DatabaseSettings {
                url: "postgres://test:test@localhost:5432/test".to_string(),
                host: "localhost".to_string(),
                port: 5432,
                name: "test".to_string(),
                username: "test".to_string(),
                password: "test".to_string(),
            },
            keycloak: KeycloakSettings {
                url: "http://localhost:8080".to_string(),
                realm_name: "test-realm".to_string(),
                client_name: "test-client".to_string(),
                admin: "admin".to_string(),
                admin_password: "admin123".to_string(),
            },
            auth: AuthSettings {
                jwt_secret: "test-secret-key-that-is-very-long-and-secure".to_string(),
                jwt_expiration_seconds: 3600,
            },
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            cache: CacheSettings {
                redis_url: "redis://localhost:6379".to_string(),
            },
            audit: AuditSettings {
                retention_days: 365,
            },
            log_level: "info".to_string(),
        }
    }

    #[test]
    fn test_generate_and_validate_token() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "admin",
            Some("company-456"),
        ).unwrap();

        let claims = jwt_service.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.company_id, Some("company-456".to_string()));
    }

    #[test]
    fn test_generate_token_without_company() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "user",
            None,
        ).unwrap();

        let claims = jwt_service.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, "user");
        assert!(claims.company_id.is_none());
    }

    #[test]
    fn test_validate_invalid_token() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let result = jwt_service.validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let original_token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "user",
            Some("company-456"),
        ).unwrap();

        let refreshed_token = jwt_service.refresh_token(&original_token).unwrap();
        
        let original_claims = jwt_service.validate_token(&original_token).unwrap();
        let refreshed_claims = jwt_service.validate_token(&refreshed_token).unwrap();

        // Claims should be the same except for expiration and jti
        assert_eq!(original_claims.sub, refreshed_claims.sub);
        assert_eq!(original_claims.username, refreshed_claims.username);
        assert_eq!(original_claims.email, refreshed_claims.email);
        assert_eq!(original_claims.role, refreshed_claims.role);
        assert_eq!(original_claims.company_id, refreshed_claims.company_id);
        
        // JTI should be different
        assert_ne!(original_claims.jti, refreshed_claims.jti);
    }

    #[test]
    fn test_get_remaining_time() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "user",
            None,
        ).unwrap();

        let remaining_time = jwt_service.get_remaining_time(&token).unwrap();
        assert!(remaining_time.num_seconds() > 0);
        assert!(remaining_time.num_seconds() <= 3600);
    }

    #[test]
    fn test_is_token_expired() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "user",
            None,
        ).unwrap();

        assert!(!jwt_service.is_token_expired(&token));
        
        // Test with invalid token
        assert!(jwt_service.is_token_expired("invalid.token"));
    }

    #[test]
    fn test_extract_user_id() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "user",
            None,
        ).unwrap();

        let user_id = jwt_service.extract_user_id(&token).unwrap();
        assert_eq!(user_id, "user-123");
    }

    #[test]
    fn test_extract_role() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "admin",
            None,
        ).unwrap();

        let role = jwt_service.extract_role(&token).unwrap();
        assert_eq!(role, "admin");
    }

    #[test]
    fn test_extract_company_id() {
        let settings = create_test_settings();
        let jwt_service = JwtService::new(&settings);
        
        let token = jwt_service.generate_token(
            "user-123",
            "testuser",
            "test@example.com",
            "user",
            Some("company-456"),
        ).unwrap();

        let company_id = jwt_service.extract_company_id(&token).unwrap();
        assert_eq!(company_id, Some("company-456".to_string()));

        // Test without company
        let token_no_company = jwt_service.generate_token(
            "user-124",
            "testuser2",
            "test2@example.com",
            "user",
            None,
        ).unwrap();

        let company_id_none = jwt_service.extract_company_id(&token_no_company).unwrap();
        assert!(company_id_none.is_none());
    }

    #[test]
    fn test_claims_serialization() {
        let claims = Claims {
            sub: "user-123".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            jti: "test-jti".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            role: "admin".to_string(),
            company_id: Some("company-456".to_string()),
        };

        // Test that claims can be serialized and deserialized
        let serialized = serde_json::to_string(&claims).unwrap();
        let deserialized: Claims = serde_json::from_str(&serialized).unwrap();

        assert_eq!(claims.sub, deserialized.sub);
        assert_eq!(claims.username, deserialized.username);
        assert_eq!(claims.email, deserialized.email);
        assert_eq!(claims.role, deserialized.role);
        assert_eq!(claims.company_id, deserialized.company_id);
    }
}