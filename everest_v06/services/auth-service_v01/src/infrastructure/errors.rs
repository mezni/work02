use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use reqwest::Error as ReqwestError;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] ReqwestError),

    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),

    #[error("Hashing error: {0}")]
    Hashing(#[from] BcryptError),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Keycloak error: {0}")]
    Keycloak(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl InfrastructureError {
    pub fn code(&self) -> &'static str {
        match self {
            InfrastructureError::Database(_) => "INFRA_DATABASE_ERROR",
            InfrastructureError::HttpClient(_) => "INFRA_HTTP_CLIENT_ERROR",
            InfrastructureError::Jwt(_) => "INFRA_JWT_ERROR",
            InfrastructureError::Hashing(_) => "INFRA_HASHING_ERROR",
            InfrastructureError::Configuration(_) => "INFRA_CONFIGURATION_ERROR",
            InfrastructureError::Keycloak(_) => "INFRA_KEYCLOAK_ERROR",
            InfrastructureError::Authentication(_) => "INFRA_AUTHENTICATION_ERROR",
            InfrastructureError::Authorization(_) => "INFRA_AUTHORIZATION_ERROR",
            InfrastructureError::Cache(_) => "INFRA_CACHE_ERROR",
            InfrastructureError::EventBus(_) => "INFRA_EVENT_BUS_ERROR",
            InfrastructureError::Io(_) => "INFRA_IO_ERROR",
            InfrastructureError::Serialization(_) => "INFRA_SERIALIZATION_ERROR",
        }
    }
}
