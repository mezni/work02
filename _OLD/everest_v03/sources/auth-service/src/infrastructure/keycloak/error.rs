// src/infrastructure/keycloak/error.rs
use thiserror::Error;
use reqwest::StatusCode;

#[derive(Error, Debug)]
pub enum KeycloakError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Keycloak API error: {status} - {message}")]
    Api {
        status: StatusCode,
        message: String,
    },
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("User not found in Keycloak")]
    UserNotFound,
    
    #[error("User already exists in Keycloak")]
    UserAlreadyExists,
    
    #[error("Invalid configuration: {0}")]
    Configuration(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl KeycloakError {
    pub fn api(status: StatusCode, message: String) -> Self {
        KeycloakError::Api { status, message }
    }
    
    pub fn authentication(message: &str) -> Self {
        KeycloakError::Authentication(message.to_string())
    }
    
    pub fn configuration(message: &str) -> Self {
        KeycloakError::Configuration(message.to_string())
    }
}