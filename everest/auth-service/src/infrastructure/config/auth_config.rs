// src/infrastructure/config/auth_config.rs
use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
}

#[derive(Error, Debug)]
pub enum AuthConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

impl AuthConfig {
    pub fn from_env() -> Result<Self, AuthConfigError> {
        dotenvy::dotenv().ok();

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| AuthConfigError::MissingEnvVar("JWT_SECRET".to_string()))?;

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse::<u64>()
            .map_err(|e| {
                AuthConfigError::InvalidConfig(format!("Invalid expiration hours: {}", e))
            })?;

        Ok(Self {
            jwt_secret,
            jwt_expiration_hours,
        })
    }
}
