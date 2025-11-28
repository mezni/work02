use crate::domain::models::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::{Username, Email};

pub struct AuthService;

impl AuthService {
    pub fn validate_credentials(user: &User, password: &str) -> Result<bool, DomainError> {
        if !user.is_active {
            return Err(DomainError::InvalidCredentials);
        }

        user.verify_password(password)
            .map_err(|_| DomainError::InvalidCredentials)
    }

    pub fn hash_password(password: &str) -> Result<String, DomainError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|_| DomainError::InvalidPassword("Failed to hash password".to_string()))
    }
}
