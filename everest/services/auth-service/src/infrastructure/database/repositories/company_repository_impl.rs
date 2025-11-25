use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::entities::Company;
use crate::domain::errors::DomainError;
use crate::domain::repositories::CompanyRepository;

pub struct CompanyRepositoryImpl {
    pool: PgPool,
}

impl CompanyRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompanyRepository for CompanyRepositoryImpl {
    // ... existing methods (create, update, delete, find_by_id, etc.) ...

    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, DomainError> {
        let query = r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies 
            WHERE name = $1
        "#;

        let row = sqlx::query(query)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find company by name: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => Ok(Some(Company {
                id: r.get("id"),
                name: r.get("name"),
                description: r.get("description"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn create(&self, company: &Company) -> Result<(), DomainError> {
        let query = r#"
            INSERT INTO companies (id, name, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        sqlx::query(query)
            .bind(company.id)
            .bind(&company.name)
            .bind(&company.description)
            .bind(company.created_by)
            .bind(company.created_at)
            .bind(company.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to create company: {}", e);
                DomainError::Validation(format!("Failed to create company: {}", e))
            })?;

        info!("Company created successfully: {}", company.id);
        Ok(())
    }

    async fn update(&self, company: &Company) -> Result<(), DomainError> {
        let query = r#"
            UPDATE companies 
            SET name = $2, description = $3, updated_at = $4
            WHERE id = $1
        "#;

        let rows_affected = sqlx::query(query)
            .bind(company.id)
            .bind(&company.name)
            .bind(&company.description)
            .bind(company.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to update company: {}", e);
                DomainError::Validation(format!("Failed to update company: {}", e))
            })?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::CompanyNotFound(company.id.to_string()));
        }

        info!("Company updated successfully: {}", company.id);
        Ok(())
    }

    async fn delete(&self, company_id: &Uuid) -> Result<(), DomainError> {
        let query = "DELETE FROM companies WHERE id = $1";

        let rows_affected = sqlx::query(query)
            .bind(company_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete company: {}", e);
                DomainError::Validation(format!("Failed to delete company: {}", e))
            })?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DomainError::CompanyNotFound(company_id.to_string()));
        }

        info!("Company deleted successfully: {}", company_id);
        Ok(())
    }

    async fn find_by_id(&self, company_id: &Uuid) -> Result<Option<Company>, DomainError> {
        let query = r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies 
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(company_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find company by ID: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => Ok(Some(Company {
                id: r.get("id"),
                name: r.get("name"),
                description: r.get("description"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<Company>, DomainError> {
        let offset = (page - 1) * page_size;
        let query = r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#;

        let rows = sqlx::query(query)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find companies: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let companies = rows
            .into_iter()
            .map(|r| Company {
                id: r.get("id"),
                name: r.get("name"),
                description: r.get("description"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect();

        Ok(companies)
    }

    async fn find_by_creator(&self, user_id: &Uuid) -> Result<Vec<Company>, DomainError> {
        let query = r#"
            SELECT id, name, description, created_by, created_at, updated_at
            FROM companies 
            WHERE created_by = $1
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find companies by creator: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let companies = rows
            .into_iter()
            .map(|r| Company {
                id: r.get("id"),
                name: r.get("name"),
                description: r.get("description"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect();

        Ok(companies)
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, DomainError> {
        let query = "SELECT 1 FROM companies WHERE name = $1";

        let result = sqlx::query(query)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to check company name existence: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        Ok(result.is_some())
    }
}
