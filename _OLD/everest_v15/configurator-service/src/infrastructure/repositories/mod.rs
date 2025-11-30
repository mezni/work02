// configurator-service/src/infrastructure/repositories/mod.rs
pub mod organization_repository;
pub mod station_repository;
pub mod unit_of_work;
pub mod user_repository;

// Re-export for easy access
pub use organization_repository::OrganizationRepositoryImpl;
pub use station_repository::StationRepositoryImpl;
pub use unit_of_work::{PgUnitOfWork, UnitOfWorkImpl};
pub use user_repository::UserRepositoryImpl;

use crate::domain::repositories::{OrganizationRepository, StationRepository, UserRepository};
use sqlx::PgPool;

/// Factory for creating repository instances
pub struct RepositoryFactory {
    pool: PgPool,
}

impl RepositoryFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn user_repository(&self) -> UserRepositoryImpl {
        UserRepositoryImpl::new(self.pool.clone())
    }

    pub fn organization_repository(&self) -> OrganizationRepositoryImpl {
        OrganizationRepositoryImpl::new(self.pool.clone())
    }

    pub fn station_repository(&self) -> StationRepositoryImpl {
        StationRepositoryImpl::new(self.pool.clone())
    }

    pub fn unit_of_work(&self) -> UnitOfWorkImpl {
        UnitOfWorkImpl::new(self.pool.clone())
    }
}
