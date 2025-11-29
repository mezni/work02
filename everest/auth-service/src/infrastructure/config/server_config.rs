use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum ServerConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, ServerConfigError> {
        dotenvy::dotenv().ok();

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| ServerConfigError::InvalidConfig(format!("Invalid port: {}", e)))?;

        Ok(Self { host, port })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}