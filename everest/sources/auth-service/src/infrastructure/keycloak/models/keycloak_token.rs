// src/infrastructure/keycloak/models/keycloak_token.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub token_type: String,
}

#[derive(Debug)]
pub struct KeycloakToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl KeycloakToken {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires_in);
        Self {
            access_token,
            refresh_token,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() >= self.expires_at
    }
}