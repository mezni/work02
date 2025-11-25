use thiserror::Error;
use crate::domain::errors::DomainError;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(#[from] InfrastructureError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Company not found: {0}")]
    CompanyNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}

impl ApplicationError {
    pub fn code(&self) -> &'static str {
        match self {
            ApplicationError::Domain(_) => "APP_DOMAIN_ERROR",
            ApplicationError::Infrastructure(_) => "APP_INFRASTRUCTURE_ERROR",
            ApplicationError::Validation(_) => "APP_VALIDATION_ERROR",
            ApplicationError::Authentication(_) => "APP_AUTHENTICATION_ERROR",
            ApplicationError::Authorization(_) => "APP_AUTHORIZATION_ERROR",
            ApplicationError::UserNotFound(_) => "USER_NOT_FOUND",
            ApplicationError::CompanyNotFound(_) => "COMPANY_NOT_FOUND",
            ApplicationError::InvalidInput(_) => "INVALID_INPUT",
            ApplicationError::BusinessRuleViolation(_) => "BUSINESS_RULE_VIOLATION",
        }
    }

    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            ApplicationError::Validation(_)
                | ApplicationError::Authentication(_)
                | ApplicationError::Authorization(_)
                | ApplicationError::UserNotFound(_)
                | ApplicationError::CompanyNotFound(_)
                | ApplicationError::InvalidInput(_)
                | ApplicationError::BusinessRuleViolation(_)
        )
    }
}