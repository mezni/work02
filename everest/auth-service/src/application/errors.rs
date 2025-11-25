use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    DomainError(#[from] crate::domain::errors::DomainError),
    
    #[error("Infrastructure error: {0}")]
    InfrastructureError(#[from] crate::infrastructure::errors::InfrastructureError),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Company not found")]
    CompanyNotFound,
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ApplicationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DomainError(_) => "APP_DOMAIN_ERROR",
            Self::InfrastructureError(_) => "APP_INFRASTRUCTURE_ERROR",
            Self::AuthenticationFailed => "APP_AUTHENTICATION_FAILED",
            Self::AuthorizationFailed(_) => "APP_AUTHORIZATION_FAILED",
            Self::InvalidToken => "APP_INVALID_TOKEN",
            Self::TokenExpired => "APP_TOKEN_EXPIRED",
            Self::UserNotFound => "APP_USER_NOT_FOUND",
            Self::CompanyNotFound => "APP_COMPANY_NOT_FOUND",
            Self::InvalidOperation(_) => "APP_INVALID_OPERATION",
            Self::ValidationError(_) => "APP_VALIDATION_ERROR",
        }
    }
}
