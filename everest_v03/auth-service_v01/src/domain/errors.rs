// src/domain/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid user data")]
    InvalidUserData,
    #[error("Internal error")]
    InternalError,
}