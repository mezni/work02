// infrastructure/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] dotenvy::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    // Add other infrastructure error types as needed
}
