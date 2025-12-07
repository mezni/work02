use sqlx::PgPool;
use crate::domain::entities::NearbyStation;
use std::sync::Arc;

#[derive(Clone)]
pub struct StationService {
    pub pool: Arc<PgPool>,
}

impl StationService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Find nearby stations within a radius (meters) and limit results
    pub async fn find_nearby_stations(
        &self,
        latitude: f64,
        longitude: f64,
        radius_meters: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<NearbyStation>, sqlx::Error> {
        // Set default values
        let radius = radius_meters.unwrap_or(5000);
        let max_limit = limit.unwrap_or(50);

        // Use query instead of query_as! to avoid compile-time macros issues
        let rows = sqlx::query_as::<_, NearbyStation>(
            r#"
            SELECT
                station_id,
                name,
                address,
                distance_meters,
                has_available_connectors,
                total_available_connectors,
                max_power_kw,
                power_tier,
                operator
            FROM find_nearby_stations($1, $2, $3, $4)
            LIMIT $4
            "#
        )
        .bind(latitude)
        .bind(longitude)
        .bind(radius)
        .bind(max_limit)
        .fetch_all(self.pool.as_ref())
        .await?;

        Ok(rows)
    }
}
