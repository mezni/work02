use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::{NearbyStation, StationRepository};
use crate::infrastructure::error::DomainError;

pub struct PostgresStationRepository {
    pool: PgPool,
}

impl PostgresStationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StationRepository for PostgresStationRepository {
    async fn find_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        radius_meters: i32,
        limit: i32,
    ) -> Result<Vec<NearbyStation>, DomainError> {
        let stations = sqlx::query_as!(
            NearbyStation,
            r#"
            SELECT * FROM find_nearby_stations($1, $2, $3, $4)
            "#,
            latitude,
            longitude,
            radius_meters,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stations)
    }
}