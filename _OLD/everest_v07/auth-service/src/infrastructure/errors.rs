use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Keycloak error: {0}")]
    KeycloakError(String),
    
    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    
    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Connection pool error: {0}")]
    PoolError(String),
}

impl InfrastructureError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DatabaseError(_) => "INFRA_DATABASE_ERROR",
            Self::ConfigError(_) => "INFRA_CONFIG_ERROR",
            Self::KeycloakError(_) => "INFRA_KEYCLOAK_ERROR",
            Self::HttpClientError(_) => "INFRA_HTTP_CLIENT_ERROR",
            Self::JwtError(_) => "INFRA_JWT_ERROR",
            Self::SerializationError(_) => "INFRA_SERIALIZATION_ERROR",
            Self::IoError(_) => "INFRA_IO_ERROR",
            Self::PoolError(_) => "INFRA_POOL_ERROR",
        }
    }
}
