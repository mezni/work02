use crate::application::dtos::*;
use crate::core::errors::{AppError, AppResult};
use crate::domain::audit::{Audit, GeoLocation};
use crate::domain::user::User;
use crate::infrastructure::repositories::{
    audit_repository::AuditRepository, user_repository::UserRepository,
};
use std::sync::Arc;

pub struct UserQueryHandler {
    user_repo: Arc<dyn UserRepository>,
    audit_repo: Arc<dyn AuditRepository>,
}

impl UserQueryHandler {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        audit_repo: Arc<dyn AuditRepository>,
    ) -> Self {
        Self {
            user_repo,
            audit_repo,
        }
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: &str) -> AppResult<UserResponse> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", user_id)))?;

        Ok(user.into())
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> AppResult<UserResponse> {
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))?;

        Ok(user.into())
    }

    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> AppResult<UserResponse> {
        let user = self
            .user_repo
            .find_by_username(username)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("User with username {} not found", username))
            })?;

        Ok(user.into())
    }

    /// Get user by Keycloak ID
    pub async fn get_user_by_keycloak_id(&self, keycloak_id: &str) -> AppResult<UserResponse> {
        let user = self
            .user_repo
            .find_by_keycloak_id(keycloak_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!("User with keycloak_id {} not found", keycloak_id))
            })?;

        Ok(user.into())
    }

    /// List users with pagination
    pub async fn list_users(&self, params: PaginationParams) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .list(params.limit, params.offset)
            .await?;

        let total = self.user_repo.count().await?;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Search users by query (searches email, username, name)
    pub async fn search_users(
        &self,
        params: SearchParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .search(&params.query, params.limit, params.offset)
            .await?;

        // Get total count for the search
        let total = users.len() as i64;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get users by network ID
    pub async fn get_users_by_network(
        &self,
        network_id: &str,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_by_network(network_id, params.limit, params.offset)
            .await?;

        let total = users.len() as i64; // Could add count_by_network to repo

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get users by station ID
    pub async fn get_users_by_station(
        &self,
        station_id: &str,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_by_station(station_id, params.limit, params.offset)
            .await?;

        let total = users.len() as i64;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get users by role
    pub async fn get_users_by_role(
        &self,
        role: &str,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_by_role(role, params.limit, params.offset)
            .await?;

        let total = users.len() as i64;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get users by source (web/internal)
    pub async fn get_users_by_source(
        &self,
        source: &str,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_by_source(source, params.limit, params.offset)
            .await?;

        let total = users.len() as i64;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get active users
    pub async fn get_active_users(
        &self,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_active(params.limit, params.offset)
            .await?;

        let total = self.user_repo.count_active().await?;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get inactive users
    pub async fn get_inactive_users(
        &self,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_inactive(params.limit, params.offset)
            .await?;

        let total = users.len() as i64;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get verified users
    pub async fn get_verified_users(
        &self,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<UserResponse>> {
        let users = self
            .user_repo
            .find_verified(params.limit, params.offset)
            .await?;

        let total = users.len() as i64;

        let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

        Ok(PaginatedResponse::new(
            user_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get user statistics
    pub async fn get_user_statistics(&self) -> AppResult<UserStatistics> {
        let total = self.user_repo.count().await?;
        let active = self.user_repo.count_active().await?;
        let verified = self.user_repo.count_verified().await?;
        let by_role = self.user_repo.count_by_role().await?;
        let by_source = self.user_repo.count_by_source().await?;

        Ok(UserStatistics {
            total,
            active,
            inactive: total - active,
            verified,
            unverified: total - verified,
            by_role,
            by_source,
        })
    }

    /// Get audit logs for a user
    pub async fn get_user_audit_logs(
        &self,
        user_id: &str,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<AuditLogResponse>> {
        let audits = self
            .audit_repo
            .find_by_user(user_id, params.limit, params.offset)
            .await?;

        let total = self.audit_repo.count_by_user(user_id).await?;

        let audit_responses: Vec<AuditLogResponse> =
            audits.into_iter().map(|a| a.into()).collect();

        Ok(PaginatedResponse::new(
            audit_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get audit logs by action
    pub async fn get_audit_logs_by_action(
        &self,
        action: &str,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<AuditLogResponse>> {
        let audits = self
            .audit_repo
            .find_by_action(action, params.limit, params.offset)
            .await?;

        let total = audits.len() as i64;

        let audit_responses: Vec<AuditLogResponse> =
            audits.into_iter().map(|a| a.into()).collect();

        Ok(PaginatedResponse::new(
            audit_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get all audit logs
    pub async fn get_all_audit_logs(
        &self,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<AuditLogResponse>> {
        let audits = self
            .audit_repo
            .list(params.limit, params.offset)
            .await?;

        let total = self.audit_repo.count().await?;

        let audit_responses: Vec<AuditLogResponse> =
            audits.into_iter().map(|a| a.into()).collect();

        Ok(PaginatedResponse::new(
            audit_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Get audit logs by date range
    pub async fn get_audit_logs_by_date_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
        params: PaginationParams,
    ) -> AppResult<PaginatedResponse<AuditLogResponse>> {
        let audits = self
            .audit_repo
            .find_by_date_range(start, end, params.limit, params.offset)
            .await?;

        let total = audits.len() as i64;

        let audit_responses: Vec<AuditLogResponse> =
            audits.into_iter().map(|a| a.into()).collect();

        Ok(PaginatedResponse::new(
            audit_responses,
            total,
            params.limit,
            params.offset,
        ))
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> AppResult<bool> {
        Ok(self.user_repo.find_by_email(email).await?.is_some())
    }

    /// Check if username exists
    pub async fn username_exists(&self, username: &str) -> AppResult<bool> {
        Ok(self.user_repo.find_by_username(username).await?.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests would go here with mocked repositories
}