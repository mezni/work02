use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    pub url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

impl KeycloakConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        let url = env::var("KEYCLOAK_URL")
            .map_err(|_| ConfigError::MissingEnvVar("KEYCLOAK_URL".to_string()))?;
        
        let realm = env::var("KEYCLOAK_REALM")
            .map_err(|_| ConfigError::MissingEnvVar("KEYCLOAK_REALM".to_string()))?;
        
        let client_id = env::var("KEYCLOAK_CLIENT_ID")
            .map_err(|_| ConfigError::MissingEnvVar("KEYCLOAK_CLIENT_ID".to_string()))?;
        
        let client_secret = env::var("KEYCLOAK_CLIENT_SECRET")
            .map_err(|_| ConfigError::MissingEnvVar("KEYCLOAK_CLIENT_SECRET".to_string()))?;
        
        let admin_username = env::var("KEYCLOAK_ADMIN_USERNAME")
            .map_err(|_| ConfigError::MissingEnvVar("KEYCLOAK_ADMIN_USERNAME".to_string()))?;
        
        let admin_password = env::var("KEYCLOAK_ADMIN_PASSWORD")
            .map_err(|_| ConfigError::MissingEnvVar("KEYCLOAK_ADMIN_PASSWORD".to_string()))?;

        Ok(Self {
            url,
            realm,
            client_id,
            client_secret,
            admin_username,
            admin_password,
        })
    }
}