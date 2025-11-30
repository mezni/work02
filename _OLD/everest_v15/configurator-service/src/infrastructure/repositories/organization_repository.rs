// configurator-service/src/infrastructure/repositories/organization_repository.rs
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    organization::{Organization, OrganizationStatistics, OrganizationSummary},
    repositories::{OrganizationRepository, RepositoryError, RepositoryResult},
    types::{AuditInfo, OrganizationId, OrganizationStatus, UserId},
};

#[derive(Debug, Clone)]
pub struct OrganizationRepositoryImpl {
    pool: PgPool,
}

impl OrganizationRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn find_by_id(&self, id: OrganizationId) -> Result<Organization, String> {
        let record = sqlx::query!(
            r#"
            SELECT id, name, status, created_by, updated_by, created_at, updated_at
            FROM organizations 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Organization {
            id: record.id,
            name: record.name,
            status: match record.status.as_str() {
                "active" => OrganizationStatus::Active,
                "inactive" => OrganizationStatus::Inactive,
                _ => return Err("Invalid organization status".to_string()),
            },
            audit: AuditInfo {
                created_by: record.created_by,
                updated_by: record.updated_by,
                created_at: record.created_at,
                updated_at: record.updated_at,
            },
        })
    }

    async fn save(&self, org: Organization) -> Result<(), String> {
        let status_str = match org.status {
            OrganizationStatus::Active => "active",
            OrganizationStatus::Inactive => "inactive",
        };

        sqlx::query!(
            r#"
            INSERT INTO organizations (id, name, status, created_by, updated_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                name = $2, status = $3, updated_by = $5, updated_at = $7
            "#,
            org.id,
            org.name,
            status_str,
            org.audit.created_by,
            org.audit.updated_by,
            org.audit.created_at,
            org.audit.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn list_all(&self, page: u32, per_page: u32) -> Result<(Vec<Organization>, u64), String> {
        let offset = (page - 1) * per_page;

        // Get total count
        let count_result = sqlx::query!(r#"SELECT COUNT(*) as count FROM organizations"#)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let total = count_result.count.unwrap_or(0) as u64;

        // Get paginated records
        let records = sqlx::query!(
            r#"
            SELECT id, name, status, created_by, updated_by, created_at, updated_at
            FROM organizations 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            per_page as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let organizations: Vec<Organization> = records
            .into_iter()
            .map(|record| Organization {
                id: record.id,
                name: record.name,
                status: match record.status.as_str() {
                    "active" => OrganizationStatus::Active,
                    "inactive" => OrganizationStatus::Inactive,
                    _ => panic!("Invalid organization status in database"),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            })
            .collect();

        Ok((organizations, total))
    }

    async fn find_by_status(
        &self,
        status: OrganizationStatus,
    ) -> Result<Vec<Organization>, String> {
        let status_str = match status {
            OrganizationStatus::Active => "active",
            OrganizationStatus::Inactive => "inactive",
        };

        let records = sqlx::query!(
            r#"
            SELECT id, name, status, created_by, updated_by, created_at, updated_at
            FROM organizations 
            WHERE status = $1
            ORDER BY name
            "#,
            status_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let organizations: Vec<Organization> = records
            .into_iter()
            .map(|record| Organization {
                id: record.id,
                name: record.name,
                status: match record.status.as_str() {
                    "active" => OrganizationStatus::Active,
                    "inactive" => OrganizationStatus::Inactive,
                    _ => panic!("Invalid organization status in database"),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            })
            .collect();

        Ok(organizations)
    }

    async fn name_exists(&self, name: &str) -> Result<bool, String> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM organizations WHERE name = $1) as "exists!"
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.exists)
    }

    async fn get_summary(&self, org_id: OrganizationId) -> Result<OrganizationSummary, String> {
        let org = self.find_by_id(org_id).await?;

        // Count users in this organization
        let user_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM users 
            WHERE organization_id = $1 AND status != 'deleted'
            "#,
            org_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Count stations in this organization
        let station_count = sqlx::query!(
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

        Ok(OrganizationSummary {
            id: org.id,
            name: org.name,
            status: org.status,
            total_users: user_count.count.unwrap_or(0) as u32,
            total_stations: station_count.count.unwrap_or(0) as u32,
            created_at: org.audit.created_at,
        })
    }

    async fn get_statistics(
        &self,
        org_id: OrganizationId,
    ) -> Result<OrganizationStatistics, String> {
        // For now, return default statistics
        // In a real implementation, this would aggregate data from multiple tables
        Ok(OrganizationStatistics::default())
    }

    async fn soft_delete(&self, org_id: OrganizationId, deleted_by: UserId) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE organizations 
            SET status = 'inactive', updated_by = $2, updated_at = NOW()
            WHERE id = $1 AND status = 'active'
            "#,
            org_id,
            deleted_by
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

// Implementation of the extension trait methods
impl OrganizationRepositoryImpl {
    pub async fn find_active_by_id(&self, id: OrganizationId) -> RepositoryResult<Organization> {
        let org = self
            .find_by_id(id)
            .await
            .map_err(|e| RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            })?;

        if org.status != OrganizationStatus::Active {
            return Err(RepositoryError::not_found("Organization", &id.to_string()));
        }

        Ok(org)
    }

    pub async fn is_name_available(&self, name: &str) -> RepositoryResult<bool> {
        self.name_exists(name)
            .await
            .map(|exists| !exists)
            .map_err(|e| RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            })
    }
}
