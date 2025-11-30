// configurator-service/src/infrastructure/repositories/station_repository.rs
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    repositories::StationRepository,
    station::{Station, StationStatus},
    types::{AuditInfo, OrganizationId, StationId, UserId},
};

#[derive(Debug, Clone)]
pub struct StationRepositoryImpl {
    pool: PgPool,
}

impl StationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StationRepository for StationRepositoryImpl {
    async fn find_by_id(&self, id: StationId) -> Result<Station, String> {
        let record = sqlx::query!(
            r#"
            SELECT id, name, location, organization_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM stations 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Station {
            id: record.id,
            name: record.name,
            location: record.location,
            organization_id: record.organization_id,
            status: match record.status.as_str() {
                "active" => StationStatus::Active,
                "inactive" => StationStatus::Inactive,
                "maintenance" => StationStatus::Maintenance,
                _ => return Err("Invalid station status".to_string()),
            },
            audit: AuditInfo {
                created_by: record.created_by,
                updated_by: record.updated_by,
                created_at: record.created_at,
                updated_at: record.updated_at,
            },
        })
    }

    async fn save(&self, station: Station) -> Result<(), String> {
        let status_str = match station.status {
            StationStatus::Active => "active",
            StationStatus::Inactive => "inactive",
            StationStatus::Maintenance => "maintenance",
        };

        sqlx::query!(
            r#"
            INSERT INTO stations (id, name, location, organization_id, status,
                                created_by, updated_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name = $2, location = $3, organization_id = $4, status = $5,
                updated_by = $7, updated_at = $9
            "#,
            station.id,
            station.name,
            station.location,
            station.organization_id,
            status_str,
            station.audit.created_by,
            station.audit.updated_by,
            station.audit.created_at,
            station.audit.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_organization(&self, org_id: OrganizationId) -> Result<Vec<Station>, String> {
        let records = sqlx::query!(
            r#"
            SELECT id, name, location, organization_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM stations 
            WHERE organization_id = $1 AND status != 'inactive'
            ORDER BY name
            "#,
            org_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let stations: Vec<Station> = records
            .into_iter()
            .map(|record| Station {
                id: record.id,
                name: record.name,
                location: record.location,
                organization_id: record.organization_id,
                status: match record.status.as_str() {
                    "active" => StationStatus::Active,
                    "inactive" => StationStatus::Inactive,
                    "maintenance" => StationStatus::Maintenance,
                    _ => panic!("Invalid station status in database"),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            })
            .collect();

        Ok(stations)
    }

    async fn find_by_status_and_organization(
        &self,
        status: StationStatus,
        org_id: OrganizationId,
    ) -> Result<Vec<Station>, String> {
        let status_str = match status {
            StationStatus::Active => "active",
            StationStatus::Inactive => "inactive",
            StationStatus::Maintenance => "maintenance",
        };

        let records = sqlx::query!(
            r#"
            SELECT id, name, location, organization_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM stations 
            WHERE status = $1 AND organization_id = $2
            ORDER BY name
            "#,
            status_str,
            org_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let stations: Vec<Station> = records
            .into_iter()
            .map(|record| Station {
                id: record.id,
                name: record.name,
                location: record.location,
                organization_id: record.organization_id,
                status: match record.status.as_str() {
                    "active" => StationStatus::Active,
                    "inactive" => StationStatus::Inactive,
                    "maintenance" => StationStatus::Maintenance,
                    _ => panic!("Invalid station status in database"),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            })
            .collect();

        Ok(stations)
    }

    async fn count_by_organization(&self, org_id: OrganizationId) -> Result<u64, String> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM stations 
            WHERE organization_id = $1 AND status != 'inactive'
            "#,
            org_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.count.unwrap_or(0) as u64)
    }

    async fn soft_delete(&self, station_id: StationId, deleted_by: UserId) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE stations 
            SET status = 'inactive', updated_by = $2, updated_at = NOW()
            WHERE id = $1 AND status != 'inactive'
            "#,
            station_id,
            deleted_by
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
