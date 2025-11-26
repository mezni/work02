use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email format")]
    InvalidEmail,
    
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    
    #[error("Invalid user role: {0}")]
    InvalidUserRole(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Company assignment not allowed for self-registered users")]
    CompanyAssignmentNotAllowed,
}