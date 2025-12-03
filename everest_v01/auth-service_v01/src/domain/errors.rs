// domain/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Password is empty")]
    EmptyPassword,
    #[error("Password exceeds maximum length of {0}")]
    ExceededMaxPasswordLength(usize),
    #[error("Hashing error")]
    HashingError,
    #[error("Invalid hash format")]
    InvalidHashFormat,
    #[error("Invalid subject")]
    InvalidSubject,
    #[error("Token creation error")]
    TokenCreationError,
    #[error("Invalid token")]
    InvalidToken,
}
