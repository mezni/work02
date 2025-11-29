use async_trait::async_trait;
use crate::domain::entities::role_request::{RoleRequest, CreateRoleRequest, ReviewRoleRequest};
use super::RepositoryError;

#[async_trait]
pub trait RoleRequestRepository: Send + Sync {
    async fn create(&self, request: &CreateRoleRequest) -> Result<i32, RepositoryError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<RoleRequest>, RepositoryError>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<RoleRequest>, RepositoryError>;
    async fn find_pending_requests(&self) -> Result<Vec<RoleRequest>, RepositoryError>;
    async fn update(&self, id: i32, review: &ReviewRoleRequest, reviewed_by: &str) -> Result<(), RepositoryError>;
}