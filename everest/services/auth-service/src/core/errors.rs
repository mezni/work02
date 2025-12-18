use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal Server Error")]
    Internal,

    #[error("Database error: {0}")]
    DatabaseError(String),
}
