use crate::domain::entities::NearbyStation;
use sqlx::PgPool;

pub struct StationRepository {
    pool: PgPool,
}

impl StationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_nearby_stations(
        &self,
        latitude: f64,
        longitude: f64,
        radius_meters: i32,
        limit: i32,
    ) -> Result<Vec<NearbyStation>, sqlx::Error> {
        let stations: Vec<NearbyStation> = sqlx::query_as::<_, NearbyStation>(
            r#"
            SELECT *
            FROM find_nearby_stations($1, $2, $3, $4)
            "#
        )
        .bind(latitude)
        .bind(longitude)
        .bind(radius_meters)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(stations)
    }
}
