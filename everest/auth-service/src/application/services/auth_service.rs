// src/application/services/auth_service.rs
use crate::application::dto::auth_dto::{LoginRequest, LoginResponse, TokenClaims};
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::config::auth_config::AuthConfig;
use crate::infrastructure::keycloak::client::KeycloakClient;
use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::sync::Arc;
use tracing::{error, info};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Token creation failed: {0}")]
    TokenCreation(String),
    #[error("Token validation failed: {0}")]
    TokenValidation(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub struct AuthService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    keycloak_client: Arc<KeycloakClient>,
    config: AuthConfig,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        keycloak_client: Arc<KeycloakClient>,
        config: AuthConfig,
    ) -> Self {
        Self {
            user_repository,
            keycloak_client,
            config,
        }
    }

    /// Authenticate user and generate JWT token
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AuthError> {
        info!("Attempting login for user: {}", request.username);

        // In a real implementation, you would validate credentials against Keycloak
        // For now, we'll check if the user exists and create a token
        let user = self
            .user_repository
            .find_by_username(&request.username)
            .await
            .map_err(|e| {
                error!("Failed to find user: {}", e);
                AuthError::Internal(e.to_string())
            })?
            .ok_or(AuthError::UserNotFound)?;

        // Validate password (in real implementation, validate against Keycloak)
        // For demo purposes, we'll skip actual password validation

        // Generate JWT token
        let user_id = user
            .id
            .as_ref()
            .ok_or_else(|| AuthError::Internal("User ID not found".to_string()))?
            .as_str()
            .to_string();

        let token = self.generate_token(&user_id, &user.username)?;

        info!("Login successful for user: {}", request.username);

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.jwt_expiration_hours * 3600,
            user_id,
            username: user.username.clone(),
        })
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<TokenClaims, AuthError> {
        let decoding_key = DecodingKey::from_secret(self.config.jwt_secret.as_ref());
        let validation = Validation::default();

        decode::<TokenClaims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| {
                error!("Token validation failed: {}", e);
                AuthError::TokenValidation(e.to_string())
            })
    }

    /// Generate JWT token
    fn generate_token(&self, user_id: &str, username: &str) -> Result<String, AuthError> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(
                self.config.jwt_expiration_hours as i64,
            ))
            .expect("Invalid timestamp")
            .timestamp();

        let claims = TokenClaims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: expiration as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };

        let encoding_key = EncodingKey::from_secret(self.config.jwt_secret.as_ref());
        encode(&Header::default(), &claims, &encoding_key)
            .map_err(|e| AuthError::TokenCreation(e.to_string()))
    }

    /// Refresh token
    pub async fn refresh_token(&self, token: &str) -> Result<LoginResponse, AuthError> {
        let claims = self.validate_token(token)?;

        // Get user to ensure they still exist
        let user = self
            .user_repository
            .find_by_username(&claims.username)
            .await
            .map_err(|e| AuthError::Internal(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        // Generate new token
        let user_id = user
            .id
            .as_ref()
            .ok_or_else(|| AuthError::Internal("User ID not found".to_string()))?
            .as_str()
            .to_string();

        let new_token = self.generate_token(&user_id, &user.username)?;

        Ok(LoginResponse {
            access_token: new_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.jwt_expiration_hours * 3600,
            user_id,
            username: user.username.clone(),
        })
    }
}
