use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Company not found: {0}")]
    CompanyNotFound(String),

    #[error("Email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("Username already exists: {0}")]
    UsernameAlreadyExists(String),

    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Invalid password: {0}")]
    InvalidPassword(String),

    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("User already in company: {0}")]
    UserAlreadyInCompany(String),

    #[error("Unauthorized access: {0}")]
    Unauthorized(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Domain rule violation: {0}")]
    BusinessRuleViolation(String),
}

impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            DomainError::Validation(_) => "DOMAIN_VALIDATION_ERROR",
            DomainError::UserNotFound(_) => "USER_NOT_FOUND",
            DomainError::CompanyNotFound(_) => "COMPANY_NOT_FOUND",
            DomainError::EmailAlreadyExists(_) => "EMAIL_ALREADY_EXISTS",
            DomainError::UsernameAlreadyExists(_) => "USERNAME_ALREADY_EXISTS",
            DomainError::InvalidEmail(_) => "INVALID_EMAIL",
            DomainError::InvalidPassword(_) => "INVALID_PASSWORD",
            DomainError::InvalidRole(_) => "INVALID_ROLE",
            DomainError::UserAlreadyInCompany(_) => "USER_ALREADY_IN_COMPANY",
            DomainError::Unauthorized(_) => "UNAUTHORIZED",
            DomainError::InvalidOperation(_) => "INVALID_OPERATION",
            DomainError::BusinessRuleViolation(_) => "BUSINESS_RULE_VIOLATION",
        }
    }
}
