use crate::domain::station::Station;
use crate::errors::AppError;
use sqlx::PgPool;

#[derive(Clone)]
pub struct StationRepo {
    pool: PgPool,
}

impl StationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, station: &Station) -> Result<Station, AppError> {
        sqlx::query!(
            "INSERT INTO stations (id, name, org_id) VALUES ($1, $2, $3)",
            station.id,
            station.name,
            station.org_id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::InternalError)?;

        Ok(station.clone())
    }
}
