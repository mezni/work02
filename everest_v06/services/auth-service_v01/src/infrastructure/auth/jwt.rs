use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::infrastructure::config::Settings;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
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

        encode(&Header::default(), &claims, &self.encoding_key).map_err(|e| {
            error!("Failed to generate JWT token: {}", e);
            InfrastructureError::Jwt(e)
        })
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, InfrastructureError> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation).map_err(|e| {
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
