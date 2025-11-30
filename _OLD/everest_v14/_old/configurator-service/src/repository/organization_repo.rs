use crate::domain::organization::Organization;
use crate::errors::AppError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct OrganizationRepo {
    pool: PgPool,
}

impl OrganizationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, org: &Organization) -> Result<Organization, AppError> {
        sqlx::query!(
            "INSERT INTO organizations (id, name, address) VALUES ($1, $2, $3)",
            org.id,
            org.name,
            org.address
        )
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::InternalError)?;

        Ok(org.clone())
    }
}
