use crate::core::errors::AppResult;
use crate::domain::entities::Station;
use crate::domain::repositories::StationRepository;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;

pub struct PgStationRepository {
    pool: PgPool,
}

impl PgStationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StationRepository for PgStationRepository {
    async fn create(&self, station: &Station) -> AppResult<Station> {
        let _result = sqlx::query(  // Add underscore prefix
            r#"
            INSERT INTO stations (
                station_id, osm_id, name, address, location, tags,
                network_id, created_by, created_at, updated_by, updated_at
            ) VALUES ($1, $2, $3, $4, ST_SetSRID(ST_MakePoint($5, $6), 4326)::geography, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(&station.station_id)
        .bind(station.osm_id)
        .bind(&station.name)
        .bind(&station.address)
        .bind(station.longitude)
        .bind(station.latitude)
        .bind(&station.tags)
        .bind(&station.network_id)
        .bind(&station.created_by)
        .bind(&station.created_at)
        .bind(&station.updated_by)
        .bind(&station.updated_at)
        .execute(&self.pool)
        .await?;

        // Fetch the created station
        self.find_by_id(&station.station_id).await?.ok_or(
            crate::core::errors::AppError::InternalError(
                "Failed to fetch created station".to_string(),
            ),
        )
    }

    async fn find_by_id(&self, station_id: &str) -> AppResult<Option<Station>> {
        let result = sqlx::query_as::<
            _,
            (
                String,
                i64,
                String,
                Option<String>,
                f64,
                f64,
                Option<serde_json::Value>,
                Option<String>,
                Option<String>,
                chrono::DateTime<Utc>,
                Option<String>,
                Option<chrono::DateTime<Utc>>,
            ),
        >(
            r#"
            SELECT 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            FROM stations WHERE station_id = $1
            "#,
        )
        .bind(station_id)
        .fetch_optional(&self.pool)
        .await?
        .map(
            |(
                station_id,
                osm_id,
                name,
                address,
                latitude,
                longitude,
                tags,
                network_id,
                created_by,
                created_at,
                updated_by,
                updated_at,
            )| {
                Station {
                    station_id,
                    osm_id,
                    name,
                    address,
                    latitude,
                    longitude,
                    tags,
                    network_id,
                    created_by,
                    created_at,
                    updated_by,
                    updated_at,
                }
            },
        );

        Ok(result)
    }

    async fn find_by_network(
        &self,
        network_id: &str,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<Station>> {
        let results = sqlx::query_as::<
            _,
            (
                String,
                i64,
                String,
                Option<String>,
                f64,
                f64,
                Option<serde_json::Value>,
                Option<String>,
                Option<String>,
                chrono::DateTime<Utc>,
                Option<String>,
                Option<chrono::DateTime<Utc>>,
            ),
        >(
            r#"
            SELECT 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            FROM stations 
            WHERE network_id = $1
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(network_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(
                station_id,
                osm_id,
                name,
                address,
                latitude,
                longitude,
                tags,
                network_id,
                created_by,
                created_at,
                updated_by,
                updated_at,
            )| {
                Station {
                    station_id,
                    osm_id,
                    name,
                    address,
                    latitude,
                    longitude,
                    tags,
                    network_id,
                    created_by,
                    created_at,
                    updated_by,
                    updated_at,
                }
            },
        )
        .collect();

        Ok(results)
    }

    async fn find_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Station>> {
        let results = sqlx::query_as::<
            _,
            (
                String,
                i64,
                String,
                Option<String>,
                f64,
                f64,
                Option<serde_json::Value>,
                Option<String>,
                Option<String>,
                chrono::DateTime<Utc>,
                Option<String>,
                Option<chrono::DateTime<Utc>>,
            ),
        >(
            r#"
            SELECT 
                station_id, osm_id, name, address,
                ST_Y(location::geometry) as latitude,
                ST_X(location::geometry) as longitude,
                tags, network_id, created_by, created_at, updated_by, updated_at
            FROM stations 
            ORDER BY created_at DESC 
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(
            |(
                station_id,
                osm_id,
                name,
                address,
                latitude,
                longitude,
                tags,
                network_id,
                created_by,
                created_at,
                updated_by,
                updated_at,
            )| {
                Station {
                    station_id,
                    osm_id,
                    name,
                    address,
                    latitude,
                    longitude,
                    tags,
                    network_id,
                    created_by,
                    created_at,
                    updated_by,
                    updated_at,
                }
            },
        )
        .collect();

        Ok(results)
    }

    async fn update(&self, station: &Station) -> AppResult<Station> {
        sqlx::query(
            r#"
            UPDATE stations SET
                name = $2,
                address = $3,
                location = ST_SetSRID(ST_MakePoint($4, $5), 4326)::geography,
                tags = $6,
                network_id = $7,
                updated_by = $8,
                updated_at = $9
            WHERE station_id = $1
            "#,
        )
        .bind(&station.station_id)
        .bind(&station.name)
        .bind(&station.address)
        .bind(station.longitude)
        .bind(station.latitude)
        .bind(&station.tags)
        .bind(&station.network_id)
        .bind(&station.updated_by)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        self.find_by_id(&station.station_id)
            .await?
            .ok_or(crate::core::errors::AppError::NotFound(
                "Station not found".to_string(),
            ))
    }

    async fn delete(&self, station_id: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM stations WHERE station_id = $1")
            .bind(station_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn count(&self) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stations")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }
}
