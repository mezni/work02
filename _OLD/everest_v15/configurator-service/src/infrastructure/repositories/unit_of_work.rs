// configurator-service/src/infrastructure/repositories/unit_of_work.rs
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::repositories::{
    OrganizationRepository, RepositoryError, RepositoryResult, StationRepository, UnitOfWork,
    UserRepository,
};

use super::{
    organization_repository::OrganizationRepositoryImpl, station_repository::StationRepositoryImpl,
    user_repository::UserRepositoryImpl,
};

/// PostgreSQL implementation of Unit of Work
pub struct PgUnitOfWork {
    pool: PgPool,
    transaction: Option<Transaction<'static, Postgres>>,
}

impl PgUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            transaction: None,
        }
    }
}

#[async_trait]
impl UnitOfWork for PgUnitOfWork {
    type UserRepo = UserRepositoryImpl;
    type OrganizationRepo = OrganizationRepositoryImpl;
    type StationRepo = StationRepositoryImpl;

    fn users(&self) -> &Self::UserRepo {
        // This would need to be adjusted to work with transactions
        // For simplicity, we're using the pool directly
        &UserRepositoryImpl::new(self.pool.clone())
    }

    fn organizations(&self) -> &Self::OrganizationRepo {
        &OrganizationRepositoryImpl::new(self.pool.clone())
    }

    fn stations(&self) -> &Self::StationRepo {
        &StationRepositoryImpl::new(self.pool.clone())
    }

    async fn begin_transaction(&self) -> RepositoryResult<()> {
        // Implementation would start a transaction
        // This is a simplified version
        Ok(())
    }

    async fn commit_transaction(&self) -> RepositoryResult<()> {
        // Implementation would commit the transaction
        Ok(())
    }

    async fn rollback_transaction(&self) -> RepositoryResult<()> {
        // Implementation would rollback the transaction
        Ok(())
    }
}

/// Simple implementation that doesn't use transactions
pub struct UnitOfWorkImpl {
    user_repo: UserRepositoryImpl,
    organization_repo: OrganizationRepositoryImpl,
    station_repo: StationRepositoryImpl,
}

impl UnitOfWorkImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: UserRepositoryImpl::new(pool.clone()),
            organization_repo: OrganizationRepositoryImpl::new(pool.clone()),
            station_repo: StationRepositoryImpl::new(pool),
        }
    }
}

#[async_trait]
impl UnitOfWork for UnitOfWorkImpl {
    type UserRepo = UserRepositoryImpl;
    type OrganizationRepo = OrganizationRepositoryImpl;
    type StationRepo = StationRepositoryImpl;

    fn users(&self) -> &Self::UserRepo {
        &self.user_repo
    }

    fn organizations(&self) -> &Self::OrganizationRepo {
        &self.organization_repo
    }

    fn stations(&self) -> &Self::StationRepo {
        &self.station_repo
    }

    async fn begin_transaction(&self) -> RepositoryResult<()> {
        // No-op for this implementation
        Ok(())
    }

    async fn commit_transaction(&self) -> RepositoryResult<()> {
        // No-op for this implementation
        Ok(())
    }

    async fn rollback_transaction(&self) -> RepositoryResult<()> {
        // No-op for this implementation
        Ok(())
    }
}
