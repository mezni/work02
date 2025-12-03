use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use reqwest;
use redis;
use jsonwebtoken;
use serde_json::json;
use std::fmt;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Keycloak connection failed: {0}")]
    KeycloakConnection(String),
    
    #[error("Keycloak API error: {0}")]
    KeycloakApi(String),
    
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    #[error("Redis connection failed: {0}")]
    RedisConnection(#[from] redis::RedisError),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Token encoding error: {0}")]
    TokenEncoding(#[from] jsonwebtoken::errors::Error),
    
    #[error("Token decoding error: {0}")]
    TokenDecoding(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl ResponseError for InfrastructureError {
    fn error_response(&self) -> HttpResponse {
        match self {
            InfrastructureError::KeycloakConnection(msg) |
            InfrastructureError::KeycloakApi(msg) |
            InfrastructureError::HealthCheckFailed(msg) => HttpResponse::BadGateway().json(json!({
                "error": "Bad Gateway",
                "message": msg,
                "code": "KEYCLOAK_ERROR"
            })),
            InfrastructureError::RedisConnection(_) |
            InfrastructureError::CacheError(msg) => HttpResponse::ServiceUnavailable().json(json!({
                "error": "Service Unavailable",
                "message": "Cache service unavailable",
                "code": "CACHE_ERROR"
            })),
            InfrastructureError::TokenEncoding(e) |
            InfrastructureError::TokenDecoding(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "message": format!("Token error: {}", msg),
                "code": "TOKEN_ERROR"
            })),
            InfrastructureError::Configuration(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "message": msg,
                "code": "CONFIG_ERROR"
            })),
            _ => HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "message": self.to_string(),
                "code": "INFRASTRUCTURE_ERROR"
            })),
        }
    }
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type InfrastructureResult<T> = std::result::Result<T, InfrastructureError>;