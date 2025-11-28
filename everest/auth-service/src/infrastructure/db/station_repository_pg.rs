use crate::domain::{
    models::Station,
    errors::DomainError,
    repositories::StationRepository,
};
use async_trait::async_trait;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

#[derive(Clone)]
pub struct StationRepositoryPg {
    pool: PgPool,
}

impl StationRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        StationRepositoryPg { pool }
    }
}

#[derive(FromRow)]
struct StationRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    organisation_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    is_active: bool,
}

impl From<StationRow> for Station {
    fn from(row: StationRow) -> Self {
        Station {
            id: row.id,
            name: row.name,
            description: row.description,
            organisation_id: row.organisation_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            is_active: row.is_active,
        }
    }
}

#[async_trait]
impl StationRepository for StationRepositoryPg {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Station>, DomainError> {
        let row = sqlx::query_as!(
            StationRow,
            r#"SELECT id, name, description, organisation_id, created_at, updated_at, is_active 
               FROM stations WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_organisation(&self, organisation_id: Uuid) -> Result<Vec<Station>, DomainError> {
        let rows = sqlx::query_as!(
            StationRow,
            r#"SELECT id, name, description, organisation_id, created_at, updated_at, is_active 
               FROM stations WHERE organisation_id = $1 AND is_active = true ORDER BY name"#,
            organisation_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn save(&self, station: &Station) -> Result<Station, DomainError> {
        let row = sqlx::query_as!(
            StationRow,
            r#"INSERT INTO stations (id, name, description, organisation_id, created_at, updated_at, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id, name, description, organisation_id, created_at, updated_at, is_active"#,
            station.id,
            station.name,
            station.description,
            station.organisation_id,
            station.created_at,
            station.updated_at,
            station.is_active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(row.into())
    }

    async fn update(&self, station: &Station) -> Result<Station, DomainError> {
        let row = sqlx::query_as!(
            StationRow,
            r#"UPDATE stations 
               SET name = $2, description = $3, organisation_id = $4, updated_at = $5, is_active = $6
               WHERE id = $1
               RETURNING id, name, description, organisation_id, created_at, updated_at, is_active"#,
            station.id,
            station.name,
            station.description,
            station.organisation_id,
            chrono::Utc::now(),
            station.is_active
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM stations WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::InternalError(e.to_string()))?;

        Ok(())
    }
}
