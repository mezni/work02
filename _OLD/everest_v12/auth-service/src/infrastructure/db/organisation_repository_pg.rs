use crate::domain::{
    models::Organisation,
    errors::DomainError,
    repositories::OrganisationRepository,
};
use async_trait::async_trait;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

#[derive(Clone)]
pub struct OrganisationRepositoryPg {
    pool: PgPool,
}

impl OrganisationRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        OrganisationRepositoryPg { pool }
    }
}

#[derive(FromRow)]
struct OrganisationRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    is_active: bool,
}

impl From<OrganisationRow> for Organisation {
    fn from(row: OrganisationRow) -> Self {
        Organisation {
            id: row.id,
            name: row.name,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
            is_active: row.is_active,
        }
    }
}

#[async_trait]
impl OrganisationRepository for OrganisationRepositoryPg {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Organisation>, DomainError> {
        let row = sqlx::query_as!(
            OrganisationRow,
            r#"SELECT id, name, description, created_at, updated_at, is_active 
               FROM organisations WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn save(&self, organisation: &Organisation) -> Result<Organisation, DomainError> {
        let row = sqlx::query_as!(
            OrganisationRow,
            r#"INSERT INTO organisations (id, name, description, created_at, updated_at, is_active)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING id, name, description, created_at, updated_at, is_active"#,
            organisation.id,
            organisation.name,
            organisation.description,
            organisation.created_at,
            organisation.updated_at,
            organisation.is_active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(row.into())
    }

    async fn update(&self, organisation: &Organisation) -> Result<Organisation, DomainError> {
        let row = sqlx::query_as!(
            OrganisationRow,
            r#"UPDATE organisations 
               SET name = $2, description = $3, updated_at = $4, is_active = $5
               WHERE id = $1
               RETURNING id, name, description, created_at, updated_at, is_active"#,
            organisation.id,
            organisation.name,
            organisation.description,
            chrono::Utc::now(),
            organisation.is_active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM organisations WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<Organisation>, DomainError> {
        let rows = sqlx::query_as!(
            OrganisationRow,
            r#"SELECT id, name, description, created_at, updated_at, is_active 
               FROM organisations WHERE is_active = true ORDER BY name"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}
