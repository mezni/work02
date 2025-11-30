use super::organization::{Organization, OrganizationStatistics, OrganizationSummary};
use super::types::{OrganizationId, StationId, UserId, UserRole, UserStatus};
use super::user::User;
use async_trait::async_trait;

/// Repository trait for User entity operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find user by ID
    async fn find_by_id(&self, id: UserId) -> Result<User, String>;

    /// Save user (create or update)
    async fn save(&self, user: User) -> Result<(), String>;

    /// Find users by organization
    async fn find_by_organization(&self, org_id: OrganizationId) -> Result<Vec<User>, String>;

    /// Find user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;

    /// Find users by role and organization
    async fn find_by_role_and_organization(
        &self,
        role: UserRole,
        org_id: OrganizationId,
    ) -> Result<Vec<User>, String>;

    /// Find users by status
    async fn find_by_status(&self, status: UserStatus) -> Result<Vec<User>, String>;

    /// Check if email exists (for uniqueness validation)
    async fn email_exists(&self, email: &str) -> Result<bool, String>;

    /// Soft delete user (mark as deleted)
    async fn soft_delete(&self, user_id: UserId, deleted_by: UserId) -> Result<(), String>;

    /// Count users by organization
    async fn count_by_organization(&self, org_id: OrganizationId) -> Result<u64, String>;
}

/// Repository trait for Organization entity operations
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    /// Find organization by ID
    async fn find_by_id(&self, id: OrganizationId) -> Result<Organization, String>;

    /// Save organization (create or update)
    async fn save(&self, org: Organization) -> Result<(), String>;

    /// List all organizations with pagination
    async fn list_all(&self, page: u32, per_page: u32) -> Result<(Vec<Organization>, u64), String>;

    /// Find organizations by status
    async fn find_by_status(
        &self,
        status: super::OrganizationStatus,
    ) -> Result<Vec<Organization>, String>;

    /// Check if organization name exists (for uniqueness validation)
    async fn name_exists(&self, name: &str) -> Result<bool, String>;

    /// Get organization summary with statistics
    async fn get_summary(&self, org_id: OrganizationId) -> Result<OrganizationSummary, String>;

    /// Get organization statistics
    async fn get_statistics(
        &self,
        org_id: OrganizationId,
    ) -> Result<OrganizationStatistics, String>;

    /// Soft delete organization (mark as inactive)
    async fn soft_delete(&self, org_id: OrganizationId, deleted_by: UserId) -> Result<(), String>;
}

/// Repository trait for Station entity operations
#[async_trait]
pub trait StationRepository: Send + Sync {
    /// Find station by ID
    async fn find_by_id(&self, id: StationId) -> Result<super::station::Station, String>;

    /// Save station (create or update)
    async fn save(&self, station: super::station::Station) -> Result<(), String>;

    /// Find stations by organization
    async fn find_by_organization(
        &self,
        org_id: OrganizationId,
    ) -> Result<Vec<super::station::Station>, String>;

    /// Find stations by status and organization
    async fn find_by_status_and_organization(
        &self,
        status: super::StationStatus,
        org_id: OrganizationId,
    ) -> Result<Vec<super::station::Station>, String>;

    /// Count stations by organization
    async fn count_by_organization(&self, org_id: OrganizationId) -> Result<u64, String>;

    /// Soft delete station (mark as inactive)
    async fn soft_delete(&self, station_id: StationId, deleted_by: UserId) -> Result<(), String>;
}

/// Unit of Work pattern for transactional operations
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    type UserRepo: UserRepository;
    type OrganizationRepo: OrganizationRepository;
    type StationRepo: StationRepository;

    /// Get user repository
    fn users(&self) -> &Self::UserRepo;

    /// Get organization repository
    fn organizations(&self) -> &Self::OrganizationRepo;

    /// Get station repository
    fn stations(&self) -> &Self::StationRepo;

    /// Begin transaction
    async fn begin_transaction(&self) -> Result<(), String>;

    /// Commit transaction
    async fn commit_transaction(&self) -> Result<(), String>;

    /// Rollback transaction
    async fn rollback_transaction(&self) -> Result<(), String>;
}

/// Repository result type for better error handling
pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Repository error type
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Entity not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Duplicate entity: {entity} with key {key}")]
    Duplicate { entity: String, key: String },

    #[error("Database error: {source}")]
    Database {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Concurrency error: {message}")]
    Concurrency { message: String },
}

impl RepositoryError {
    pub fn not_found(entity: &str, id: &str) -> Self {
        Self::NotFound {
            entity: entity.to_string(),
            id: id.to_string(),
        }
    }

    pub fn duplicate(entity: &str, key: &str) -> Self {
        Self::Duplicate {
            entity: entity.to_string(),
            key: key.to_string(),
        }
    }

    pub fn validation(message: &str) -> Self {
        Self::Validation {
            message: message.to_string(),
        }
    }

    pub fn concurrency(message: &str) -> Self {
        Self::Concurrency {
            message: message.to_string(),
        }
    }
}

// Extension traits for common repository operations
pub trait UserRepositoryExt: UserRepository {
    /// Find active user by ID
    async fn find_active_by_id(&self, id: UserId) -> RepositoryResult<User> {
        let user = self
            .find_by_id(id)
            .await
            .map_err(|e| RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            })?;

        if user.status == super::UserStatus::Deleted {
            return Err(RepositoryError::not_found("User", &id.to_string()));
        }

        Ok(user)
    }

    /// Find active user by email
    async fn find_active_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        match self.find_by_email(email).await {
            Ok(Some(user)) if user.status != super::UserStatus::Deleted => Ok(Some(user)),
            Ok(Some(_)) => Ok(None), // User exists but is deleted
            Ok(None) => Ok(None),
            Err(e) => Err(RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }),
        }
    }
}

pub trait OrganizationRepositoryExt: OrganizationRepository {
    /// Find active organization by ID
    async fn find_active_by_id(&self, id: OrganizationId) -> RepositoryResult<Organization> {
        let org = self
            .find_by_id(id)
            .await
            .map_err(|e| RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            })?;

        if org.status != super::OrganizationStatus::Active {
            return Err(RepositoryError::not_found("Organization", &id.to_string()));
        }

        Ok(org)
    }

    /// Check if organization name is available
    async fn is_name_available(&self, name: &str) -> RepositoryResult<bool> {
        self.name_exists(name)
            .await
            .map(|exists| !exists)
            .map_err(|e| RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            })
    }
}

// Auto-implement extension traits for all types that implement the base traits
impl<T: UserRepository> UserRepositoryExt for T {}
impl<T: OrganizationRepository> OrganizationRepositoryExt for T {}

// Unit tests for repository traits
#[cfg(test)]
pub mod tests {
    use super::*;
    use uuid::Uuid;

    // Mock implementations for testing
    pub struct MockUserRepository;

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, _id: UserId) -> Result<User, String> {
            unimplemented!()
        }

        async fn save(&self, _user: User) -> Result<(), String> {
            unimplemented!()
        }

        async fn find_by_organization(&self, _org_id: OrganizationId) -> Result<Vec<User>, String> {
            unimplemented!()
        }

        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, String> {
            unimplemented!()
        }

        async fn find_by_role_and_organization(
            &self,
            _role: UserRole,
            _org_id: OrganizationId,
        ) -> Result<Vec<User>, String> {
            unimplemented!()
        }

        async fn find_by_status(&self, _status: UserStatus) -> Result<Vec<User>, String> {
            unimplemented!()
        }

        async fn email_exists(&self, _email: &str) -> Result<bool, String> {
            unimplemented!()
        }

        async fn soft_delete(&self, _user_id: UserId, _deleted_by: UserId) -> Result<(), String> {
            unimplemented!()
        }

        async fn count_by_organization(&self, _org_id: OrganizationId) -> Result<u64, String> {
            unimplemented!()
        }
    }

    #[test]
    fn test_repository_error_creation() {
        let not_found = RepositoryError::not_found("User", "123");
        assert!(matches!(not_found, RepositoryError::NotFound { .. }));

        let duplicate = RepositoryError::duplicate("User", "email@example.com");
        assert!(matches!(duplicate, RepositoryError::Duplicate { .. }));

        let validation = RepositoryError::validation("Invalid input");
        assert!(matches!(validation, RepositoryError::Validation { .. }));

        let concurrency = RepositoryError::concurrency("Version conflict");
        assert!(matches!(concurrency, RepositoryError::Concurrency { .. }));
    }

    #[test]
    fn test_repository_error_display() {
        let error = RepositoryError::not_found("User", "123");
        assert!(error.to_string().contains("User"));
        assert!(error.to_string().contains("123"));
    }
}
