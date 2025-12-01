// domain/password_service.rs
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::domain::errors::DomainError;

const MAX_PASSWORD_LENGTH: usize = 64;

pub struct PasswordService;

impl PasswordService {
    pub fn hash(password: impl Into<String>) -> Result<String, DomainError> {
        let password = password.into();

        if password.is_empty() {
            return Err(DomainError::EmptyPassword);
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(DomainError::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
        }

        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| DomainError::HashingError)?
            .to_string();

        Ok(hashed_password)
    }

    pub fn compare(password: &str, hashed_password: &str) -> Result<bool, DomainError> {
        if password.is_empty() {
            return Err(DomainError::EmptyPassword);
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(DomainError::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
        }

        let parsed_hash =
            PasswordHash::new(hashed_password).map_err(|_| DomainError::InvalidHashFormat)?;

        let password_matches = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true);

        Ok(password_matches)
    }
}
