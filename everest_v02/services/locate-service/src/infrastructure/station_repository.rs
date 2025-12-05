use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::{Station, StationRepository};
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
    async fn find_all(&self) -> Result<Vec<Station>, DomainError> {
        let stations = sqlx::query_as!(
            Station,
            r#"
            SELECT 
                station_id as "station_id!",
                network_id as "network_id!",
                name as "name!",
                address as "address!",
                city,
                state,
                country as "country!",
                latitude as "latitude: f64",
                longitude as "longitude: f64",
                total_ports as "total_ports!",
                status as "status!",
                created_at as "created_at!",
                updated_at as "updated_at!"
            FROM stations
            WHERE status = 'active'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stations)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Station>, DomainError> {
        let station = sqlx::query_as!(
            Station,
            r#"
            SELECT 
                station_id as "station_id!",
                network_id as "network_id!",
                name as "name!",
                address as "address!",
                city,
                state,
                country as "country!",
                latitude as "latitude: f64",
                longitude as "longitude: f64",
                total_ports as "total_ports!",
                status as "status!",
                created_at as "created_at!",
                updated_at as "updated_at!"
            FROM stations
            WHERE station_id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(station)
    }

    async fn find_by_city(&self, city: &str) -> Result<Vec<Station>, DomainError> {
        let stations = sqlx::query_as!(
            Station,
            r#"
            SELECT 
                station_id as "station_id!",
                network_id as "network_id!",
                name as "name!",
                address as "address!",
                city,
                state,
                country as "country!",
                latitude as "latitude: f64",
                longitude as "longitude: f64",
                total_ports as "total_ports!",
                status as "status!",
                created_at as "created_at!",
                updated_at as "updated_at!"
            FROM stations
            WHERE city ILIKE $1 AND status = 'active'
            ORDER BY created_at DESC
            "#,
            format!("%{}%", city)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stations)
    }
}