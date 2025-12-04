use std::fmt;

#[derive(Debug)]
pub enum InfrastructureError {
    KeycloakError(String),
    HttpError(String),
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfrastructureError::KeycloakError(msg) => write!(f, "KeycloakError: {}", msg),
            InfrastructureError::HttpError(msg) => write!(f, "HttpError: {}", msg),
        }
    }
}

impl std::error::Error for InfrastructureError {}
