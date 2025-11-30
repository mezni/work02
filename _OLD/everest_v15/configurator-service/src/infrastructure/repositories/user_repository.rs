// configurator-service/src/infrastructure/repositories/user_repository.rs
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{
    repositories::{RepositoryError, RepositoryResult, UserRepository},
    types::{AuditInfo, OrganizationId, UserId, UserRole, UserStatus},
    user::User,
};

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: UserId) -> Result<User, String> {
        let record = sqlx::query!(
            r#"
            SELECT id, email, display_name, role, organization_id, station_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM users 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Convert database record to domain model
        Ok(User {
            id: record.id,
            email: record.email,
            display_name: record.display_name,
            role: match record.role.as_str() {
                "super_admin" => UserRole::SuperAdmin,
                "partner" => UserRole::Partner,
                "operator" => UserRole::Operator,
                _ => return Err("Invalid user role".to_string()),
            },
            organization_id: record.organization_id,
            station_id: record.station_id,
            status: match record.status.as_str() {
                "pending" => UserStatus::Pending,
                "active" => UserStatus::Active,
                "inactive" => UserStatus::Inactive,
                "deleted" => UserStatus::Deleted,
                _ => return Err("Invalid user status".to_string()),
            },
            audit: AuditInfo {
                created_by: record.created_by,
                updated_by: record.updated_by,
                created_at: record.created_at,
                updated_at: record.updated_at,
            },
        })
    }

    async fn save(&self, user: User) -> Result<(), String> {
        let role_str = match user.role {
            UserRole::SuperAdmin => "super_admin",
            UserRole::Partner => "partner",
            UserRole::Operator => "operator",
        };

        let status_str = match user.status {
            UserStatus::Pending => "pending",
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Deleted => "deleted",
        };

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, display_name, role, organization_id, station_id, status,
                             created_by, updated_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                email = $2, display_name = $3, role = $4, organization_id = $5,
                station_id = $6, status = $7, updated_by = $9, updated_at = $11
            "#,
            user.id,
            user.email,
            user.display_name,
            role_str,
            user.organization_id,
            user.station_id,
            status_str,
            user.audit.created_by,
            user.audit.updated_by,
            user.audit.created_at,
            user.audit.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn find_by_organization(&self, org_id: OrganizationId) -> Result<Vec<User>, String> {
        let records = sqlx::query!(
            r#"
            SELECT id, email, display_name, role, organization_id, station_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM users 
            WHERE organization_id = $1 AND status != 'deleted'
            "#,
            org_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for record in records {
            users.push(User {
                id: record.id,
                email: record.email,
                display_name: record.display_name,
                role: match record.role.as_str() {
                    "super_admin" => UserRole::SuperAdmin,
                    "partner" => UserRole::Partner,
                    "operator" => UserRole::Operator,
                    _ => return Err("Invalid user role".to_string()),
                },
                organization_id: record.organization_id,
                station_id: record.station_id,
                status: match record.status.as_str() {
                    "pending" => UserStatus::Pending,
                    "active" => UserStatus::Active,
                    "inactive" => UserStatus::Inactive,
                    "deleted" => UserStatus::Deleted,
                    _ => return Err("Invalid user status".to_string()),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            });
        }

        Ok(users)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let record = sqlx::query!(
            r#"
            SELECT id, email, display_name, role, organization_id, station_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM users 
            WHERE email = $1
            "#,
            email.to_lowercase()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match record {
            Some(record) => Ok(Some(User {
                id: record.id,
                email: record.email,
                display_name: record.display_name,
                role: match record.role.as_str() {
                    "super_admin" => UserRole::SuperAdmin,
                    "partner" => UserRole::Partner,
                    "operator" => UserRole::Operator,
                    _ => return Err("Invalid user role".to_string()),
                },
                organization_id: record.organization_id,
                station_id: record.station_id,
                status: match record.status.as_str() {
                    "pending" => UserStatus::Pending,
                    "active" => UserStatus::Active,
                    "inactive" => UserStatus::Inactive,
                    "deleted" => UserStatus::Deleted,
                    _ => return Err("Invalid user status".to_string()),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            })),
            None => Ok(None),
        }
    }

    async fn find_by_role_and_organization(
        &self,
        role: UserRole,
        org_id: OrganizationId,
    ) -> Result<Vec<User>, String> {
        let role_str = match role {
            UserRole::SuperAdmin => "super_admin",
            UserRole::Partner => "partner",
            UserRole::Operator => "operator",
        };

        let records = sqlx::query!(
            r#"
            SELECT id, email, display_name, role, organization_id, station_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM users 
            WHERE role = $1 AND organization_id = $2 AND status != 'deleted'
            "#,
            role_str,
            org_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for record in records {
            users.push(User {
                id: record.id,
                email: record.email,
                display_name: record.display_name,
                role: match record.role.as_str() {
                    "super_admin" => UserRole::SuperAdmin,
                    "partner" => UserRole::Partner,
                    "operator" => UserRole::Operator,
                    _ => return Err("Invalid user role".to_string()),
                },
                organization_id: record.organization_id,
                station_id: record.station_id,
                status: match record.status.as_str() {
                    "pending" => UserStatus::Pending,
                    "active" => UserStatus::Active,
                    "inactive" => UserStatus::Inactive,
                    "deleted" => UserStatus::Deleted,
                    _ => return Err("Invalid user status".to_string()),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            });
        }

        Ok(users)
    }

    async fn find_by_status(&self, status: UserStatus) -> Result<Vec<User>, String> {
        let status_str = match status {
            UserStatus::Pending => "pending",
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Deleted => "deleted",
        };

        let records = sqlx::query!(
            r#"
            SELECT id, email, display_name, role, organization_id, station_id, status,
                   created_by, updated_by, created_at, updated_at
            FROM users 
            WHERE status = $1
            "#,
            status_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for record in records {
            users.push(User {
                id: record.id,
                email: record.email,
                display_name: record.display_name,
                role: match record.role.as_str() {
                    "super_admin" => UserRole::SuperAdmin,
                    "partner" => UserRole::Partner,
                    "operator" => UserRole::Operator,
                    _ => return Err("Invalid user role".to_string()),
                },
                organization_id: record.organization_id,
                station_id: record.station_id,
                status: match record.status.as_str() {
                    "pending" => UserStatus::Pending,
                    "active" => UserStatus::Active,
                    "inactive" => UserStatus::Inactive,
                    "deleted" => UserStatus::Deleted,
                    _ => return Err("Invalid user status".to_string()),
                },
                audit: AuditInfo {
                    created_by: record.created_by,
                    updated_by: record.updated_by,
                    created_at: record.created_at,
                    updated_at: record.updated_at,
                },
            });
        }

        Ok(users)
    }

    async fn email_exists(&self, email: &str) -> Result<bool, String> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!"
            "#,
            email.to_lowercase()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.exists)
    }

    async fn soft_delete(&self, user_id: UserId, deleted_by: UserId) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET status = 'deleted', updated_by = $2, updated_at = NOW()
            WHERE id = $1 AND status != 'deleted'
            "#,
            user_id,
            deleted_by
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn count_by_organization(&self, org_id: OrganizationId) -> Result<u64, String> {
        let result = sqlx::query!(
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

        Ok(result.count.unwrap_or(0) as u64)
    }
}

// Implementation of the extension trait methods
impl UserRepositoryImpl {
    pub async fn find_active_by_id(&self, id: UserId) -> RepositoryResult<User> {
        let user = self
            .find_by_id(id)
            .await
            .map_err(|e| RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            })?;

        if user.status == UserStatus::Deleted {
            return Err(RepositoryError::not_found("User", &id.to_string()));
        }

        Ok(user)
    }

    pub async fn find_active_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        match self.find_by_email(email).await {
            Ok(Some(user)) if user.status != UserStatus::Deleted => Ok(Some(user)),
            Ok(Some(_)) => Ok(None), // User exists but is deleted
            Ok(None) => Ok(None),
            Err(e) => Err(RepositoryError::Database {
                source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn create_test_pool() -> PgPool {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://ev_charging:password@localhost:5432/ev_charging_configurator".to_string()
        });

        PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to create test pool")
    }

    #[tokio::test]
    #[ignore = "requires database"]
    async fn test_user_repository_operations() {
        let pool = create_test_pool().await;
        let repo = UserRepositoryImpl::new(pool);

        // Test data
        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();

        // Create test user
        let user = User::new(
            "test@example.com".to_string(),
            "Test User".to_string(),
            UserRole::Partner,
            Some(org_id),
            None,
            creator_id,
        )
        .unwrap();

        // Test save
        assert!(repo.save(user.clone()).await.is_ok());

        // Test find_by_id
        let found_user = repo.find_by_id(user.id).await.unwrap();
        assert_eq!(found_user.email, "test@example.com");

        // Test email_exists
        assert!(repo.email_exists("test@example.com").await.unwrap());
        assert!(!repo.email_exists("nonexistent@example.com").await.unwrap());

        // Test find_by_organization
        let org_users = repo.find_by_organization(org_id).await.unwrap();
        assert!(!org_users.is_empty());

        // Test soft_delete
        assert!(repo.soft_delete(user.id, creator_id).await.is_ok());

        let deleted_user = repo.find_by_id(user.id).await.unwrap();
        assert_eq!(deleted_user.status, UserStatus::Deleted);
    }
}
