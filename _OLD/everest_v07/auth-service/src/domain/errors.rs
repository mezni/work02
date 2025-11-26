use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Company not found")]
    CompanyNotFound,
    
    #[error("Invalid user role: {0}")]
    InvalidUserRole(String),
    
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    
    #[error("Company already exists: {0}")]
    CompanyAlreadyExists(String),
    
    #[error("Invalid company assignment")]
    InvalidCompanyAssignment,
    
    #[error("Unauthorized operation")]
    UnauthorizedOperation,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidEmail(_) => "DOMAIN_INVALID_EMAIL",
            Self::InvalidPassword(_) => "DOMAIN_INVALID_PASSWORD",
            Self::UserNotFound => "DOMAIN_USER_NOT_FOUND",
            Self::CompanyNotFound => "DOMAIN_COMPANY_NOT_FOUND",
            Self::InvalidUserRole(_) => "DOMAIN_INVALID_USER_ROLE",
            Self::UserAlreadyExists(_) => "DOMAIN_USER_ALREADY_EXISTS",
            Self::CompanyAlreadyExists(_) => "DOMAIN_COMPANY_ALREADY_EXISTS",
            Self::InvalidCompanyAssignment => "DOMAIN_INVALID_COMPANY_ASSIGNMENT",
            Self::UnauthorizedOperation => "DOMAIN_UNAUTHORIZED_OPERATION",
            Self::ValidationError(_) => "DOMAIN_VALIDATION_ERROR",
        }
    }
}
