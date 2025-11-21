use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    Individual,
    Company,
}

impl NetworkType {
    pub fn from_str(value: &str) -> Result<Self, NetworkError> {
        match value.to_uppercase().as_str() {
            "INDIVIDUAL" => Ok(Self::Individual),
            "COMPANY" => Ok(Self::Company),
            _ => Err(NetworkError::InvalidNetworkType(value.into())),
        }
    }
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Invalid network type: {0}")]
    InvalidNetworkType(String),

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Invalid phone number")]
    InvalidPhone,

    #[error("Cannot activate inactive network")]
    CannotActivate,

    #[error("Cannot deactivate already inactive network")]
    CannotDeactivate,
}
