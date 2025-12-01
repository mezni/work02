// error.rs
use thiserror::Error as ThisError;

use crate::domain::errors::DomainError;
use crate::infrastructure::errors::InfrastructureError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Infrastructure error: {0}")]
    InfrastructureError(#[from] InfrastructureError),
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
    #[error("Other error: {0}")]
    OtherError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::InfrastructureError(InfrastructureError::IoError(e))
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::OtherError(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::OtherError(s)
    }
}
