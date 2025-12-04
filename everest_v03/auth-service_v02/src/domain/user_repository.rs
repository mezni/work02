use crate::domain::errors::DomainError;
use crate::domain::user::User;

/// Pure domain repository trait for User aggregate
#[allow(unused)]
pub trait UserRepository {
    /// Save a new user
    fn save(&self, user: &User) -> Result<(), DomainError>;

    /// Update an existing user
    fn update(&self, user: &User) -> Result<(), DomainError>;

    /// Find a user by id
    fn find_by_id(&self, id: &str) -> Result<User, DomainError>;

    /// Find a user by username
    fn find_by_username(&self, username: &str) -> Result<User, DomainError>;

    /// Check if a username already exists
    fn exists_by_username(&self, username: &str) -> Result<bool, DomainError>;

    /// Check if an email already exists
    fn exists_by_email(&self, email: &str) -> Result<bool, DomainError>;
}
