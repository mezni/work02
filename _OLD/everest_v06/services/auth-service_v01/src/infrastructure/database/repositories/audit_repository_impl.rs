use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::str::FromStr;
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::entities::AuditLog;
use crate::domain::enums::AuditAction;
use crate::domain::errors::DomainError;
use crate::domain::repositories::AuditRepository;

pub struct AuditRepositoryImpl {
    pool: PgPool,
}

impl AuditRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for AuditRepositoryImpl {
    async fn create(&self, audit_log: &AuditLog) -> Result<(), DomainError> {
        let query = r#"
            INSERT INTO audit_logs (
                id, action, user_id, user_email, user_role, company_id, 
                resource_type, resource_id, old_values, new_values, 
                ip_address, user_agent, request_path, request_method, 
                status_code, error_message, metadata, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
        "#;

        sqlx::query(query)
            .bind(audit_log.id)
            .bind(audit_log.action.to_string())
            .bind(&audit_log.user_id)
            .bind(&audit_log.user_email)
            .bind(&audit_log.user_role)
            .bind(audit_log.company_id)
            .bind(&audit_log.resource_type)
            .bind(&audit_log.resource_id)
            .bind(&audit_log.old_values)
            .bind(&audit_log.new_values)
            .bind(&audit_log.ip_address)
            .bind(&audit_log.user_agent)
            .bind(&audit_log.request_path)
            .bind(&audit_log.request_method)
            .bind(audit_log.status_code as i16)
            .bind(&audit_log.error_message)
            .bind(&audit_log.metadata)
            .bind(audit_log.created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to create audit log: {}", e);
                DomainError::Validation(format!("Failed to create audit log: {}", e))
            })?;

        info!("Audit log created successfully: {}", audit_log.id);
        Ok(())
    }

    async fn find_by_id(&self, audit_id: Uuid) -> Result<Option<AuditLog>, DomainError> {
        let query = r#"
            SELECT * FROM audit_logs WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(audit_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find audit log by ID: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        match row {
            Some(r) => Ok(Some(map_row_to_audit_log(r)?)),
            None => Ok(None),
        }
    }

    async fn find_by_user_id(
        &self,
        user_id: String,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError> {
        let offset = (page - 1) * page_size;
        let query = r#"
            SELECT * FROM audit_logs 
            WHERE user_id = $1 
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
        "#;

        let rows = sqlx::query(query)
            .bind(user_id)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find audit logs by user ID: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let mut audit_logs = Vec::new();
        for row in rows {
            audit_logs.push(map_row_to_audit_log(row)?);
        }

        Ok(audit_logs)
    }

    async fn find_by_company_id(
        &self,
        company_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError> {
        let offset = (page - 1) * page_size;
        let query = r#"
            SELECT * FROM audit_logs 
            WHERE company_id = $1 
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
        "#;

        let rows = sqlx::query(query)
            .bind(company_id)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find audit logs by company ID: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let mut audit_logs = Vec::new();
        for row in rows {
            audit_logs.push(map_row_to_audit_log(row)?);
        }

        Ok(audit_logs)
    }

    async fn find_by_action(
        &self,
        action: AuditAction,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError> {
        let offset = (page - 1) * page_size;
        let query = r#"
            SELECT * FROM audit_logs 
            WHERE action = $1 
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
        "#;

        let rows = sqlx::query(query)
            .bind(action.to_string())
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to find audit logs by action: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let mut audit_logs = Vec::new();
        for row in rows {
            audit_logs.push(map_row_to_audit_log(row)?);
        }

        Ok(audit_logs)
    }

    async fn search(
        &self,
        user_id: Option<String>,
        company_id: Option<Uuid>,
        action: Option<AuditAction>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<AuditLog>, DomainError> {
        let offset = (page - 1) * page_size;

        // Build dynamic query
        let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM audit_logs WHERE 1=1");

        if let Some(user_id) = user_id {
            query_builder.push(" AND user_id = ");
            query_builder.push_bind(user_id);
        }

        if let Some(company_id) = company_id {
            query_builder.push(" AND company_id = ");
            query_builder.push_bind(company_id);
        }

        if let Some(action) = action {
            query_builder.push(" AND action = ");
            query_builder.push_bind(action.to_string());
        }

        if let Some(start_date) = start_date {
            query_builder.push(" AND created_at >= ");
            query_builder.push_bind(start_date);
        }

        if let Some(end_date) = end_date {
            query_builder.push(" AND created_at <= ");
            query_builder.push_bind(end_date);
        }

        query_builder.push(" ORDER BY created_at DESC LIMIT ");
        query_builder.push_bind(page_size as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        let rows = query_builder
            .build()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to search audit logs: {}", e);
                DomainError::Validation(format!("Database error: {}", e))
            })?;

        let mut audit_logs = Vec::new();
        for row in rows {
            audit_logs.push(map_row_to_audit_log(row)?);
        }

        Ok(audit_logs)
    }
}

fn map_row_to_audit_log(row: sqlx::postgres::PgRow) -> Result<AuditLog, DomainError> {
    let action_str: String = row.get("action");
    let action = AuditAction::from_str(&action_str)
        .map_err(|_| DomainError::Validation(format!("Invalid audit action: {}", action_str)))?;

    Ok(AuditLog {
        id: row.get("id"),
        action,
        user_id: row.get("user_id"),
        user_email: row.get("user_email"),
        user_role: row.get("user_role"),
        company_id: row.get("company_id"),
        resource_type: row.get("resource_type"),
        resource_id: row.get("resource_id"),
        old_values: row.get("old_values"),
        new_values: row.get("new_values"),
        ip_address: row.get("ip_address"),
        user_agent: row.get("user_agent"),
        request_path: row.get("request_path"),
        request_method: row.get("request_method"),
        status_code: row.get::<i16, _>("status_code") as u16,
        error_message: row.get("error_message"),
        metadata: row.get("metadata"),
        created_at: row.get("created_at"),
    })
}
