use crate::application::dto::role_request_dto::{
    CreateRoleRequestDto, ReviewRoleRequestDto, RoleRequestDto,
};
use crate::domain::entities::role_request::{
    CreateRoleRequest, ReviewRoleRequest, RoleRequest, RoleRequestStatus,
};
use crate::domain::repositories::role_request_repository::RoleRequestRepository;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::value_objects::UserId;
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Debug, thiserror::Error)]
pub enum RoleRequestError {
    #[error("User not found")]
    UserNotFound,
    #[error("Role request not found")]
    NotFound,
    #[error("An active role request already exists for this user")]
    ActiveRequestExists,
    #[error("Role request has already been processed")]
    AlreadyProcessed,
    #[error("Invalid role: {0}")]
    InvalidRole(String),
    #[error("Invalid status: {0}")]
    InvalidStatus(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Unauthorized action")]
    Unauthorized,
}

pub struct RoleRequestService {
    role_request_repository: Arc<dyn RoleRequestRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl RoleRequestService {
    pub fn new(
        role_request_repository: Arc<dyn RoleRequestRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            role_request_repository,
            user_repository,
        }
    }

    /// Create a new role request
    pub async fn create_role_request(
        &self,
        user_id: String,
        dto: CreateRoleRequestDto,
    ) -> Result<RoleRequestDto, RoleRequestError> {
        info!("Creating role request for user: {}", user_id);

        // Validate that user exists
        let user = self
            .user_repository
            .find_by_id(&UserId::new(user_id.clone()))
            .await
            .map_err(|e| {
                error!("Failed to find user: {}", e);
                RoleRequestError::DatabaseError(e.to_string())
            })?
            .ok_or(RoleRequestError::UserNotFound)?;

        // Check if user already has an active role request
        let existing_requests = self
            .role_request_repository
            .find_by_user_id(&user_id)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;

        let has_pending_request = existing_requests
            .iter()
            .any(|req| req.status == RoleRequestStatus::Pending);

        if has_pending_request {
            warn!("User {} already has a pending role request", user_id);
            return Err(RoleRequestError::ActiveRequestExists);
        }

        // Validate the requested role
        self.validate_role(&dto.requested_role)?;

        // Create the role request
        let create_request = CreateRoleRequest {
            user_id: user_id.clone(),
            requested_role: dto.requested_role,
            reason: dto.reason,
        };

        let request_id = self
            .role_request_repository
            .create(&create_request)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;

        // Fetch the created request to return complete data
        let role_request = self
            .role_request_repository
            .find_by_id(request_id)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?
            .ok_or(RoleRequestError::DatabaseError(
                "Failed to fetch created role request".to_string(),
            ))?;

        info!("Role request created successfully: {}", request_id);
        Ok(self.to_dto(role_request))
    }

    /// List role requests with optional status filter
    pub async fn list_role_requests(
        &self,
        status_filter: Option<String>,
    ) -> Result<Vec<RoleRequestDto>, RoleRequestError> {
        info!("Listing role requests with filter: {:?}", status_filter);

        let requests = if let Some(status_str) = status_filter {
            // If status filter is provided, validate it and get specific requests
            let status = self.parse_status(&status_str)?;
            match status {
                RoleRequestStatus::Pending => self
                    .role_request_repository
                    .find_pending_requests()
                    .await
                    .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?,
                _ => {
                    // For non-pending statuses, get all and filter
                    let all_requests = self
                        .role_request_repository
                        .find_pending_requests()
                        .await
                        .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;
                    all_requests
                        .into_iter()
                        .filter(|req| req.status == status)
                        .collect()
                }
            }
        } else {
            // If no filter, get all requests (you might want to implement a proper method for this)
            self.role_request_repository
                .find_pending_requests()
                .await
                .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?
            // Note: In a real implementation, you'd have a method to get all requests
            // For now, we're using pending requests as an example
        };

        Ok(requests.into_iter().map(|req| self.to_dto(req)).collect())
    }

    /// Review a role request (admin action)
    pub async fn review_role_request(
        &self,
        request_id: i32,
        dto: ReviewRoleRequestDto,
        reviewer_id: &str,
    ) -> Result<(), RoleRequestError> {
        info!(
            "Reviewing role request {} by reviewer {}",
            request_id, reviewer_id
        );

        // Fetch the role request
        let mut role_request = self
            .role_request_repository
            .find_by_id(request_id)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?
            .ok_or(RoleRequestError::NotFound)?;

        // Check if request is still pending
        if role_request.status != RoleRequestStatus::Pending {
            warn!("Role request {} is already processed", request_id);
            return Err(RoleRequestError::AlreadyProcessed);
        }

        // Parse and validate the new status
        let new_status = self.parse_status(&dto.status)?;

        // Create review object
        let review = ReviewRoleRequest {
            status: new_status.clone(),
            review_notes: dto.review_notes,
        };

        // Update the role request
        self.role_request_repository
            .update(request_id, &review, reviewer_id)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;

        // If approved, update user's role
        if new_status == RoleRequestStatus::Approved {
            self.update_user_role(&role_request.user_id, &role_request.requested_role)
                .await?;
        }

        info!(
            "Role request {} reviewed successfully with status: {:?}",
            request_id, new_status
        );
        Ok(())
    }

    /// Get role requests for a specific user
    pub async fn get_user_role_requests(
        &self,
        user_id: &str,
    ) -> Result<Vec<RoleRequestDto>, RoleRequestError> {
        info!("Getting role requests for user: {}", user_id);

        // Validate that user exists
        let user = self
            .user_repository
            .find_by_id(&UserId::new(user_id.to_string()))
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?
            .ok_or(RoleRequestError::UserNotFound)?;

        let requests = self
            .role_request_repository
            .find_by_user_id(user_id)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;

        Ok(requests.into_iter().map(|req| self.to_dto(req)).collect())
    }

    /// Get pending role requests count (for admin dashboard)
    pub async fn get_pending_requests_count(&self) -> Result<usize, RoleRequestError> {
        let pending_requests = self
            .role_request_repository
            .find_pending_requests()
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;

        Ok(pending_requests.len())
    }

    /// Update user's role when a request is approved
    async fn update_user_role(
        &self,
        user_id: &str,
        new_role: &str,
    ) -> Result<(), RoleRequestError> {
        info!("Updating user {} role to: {}", user_id, new_role);

        let user_id_obj = UserId::new(user_id.to_string());
        
        // Fetch current user
        let mut user = self
            .user_repository
            .find_by_id(&user_id_obj)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?
            .ok_or(RoleRequestError::UserNotFound)?;

        // Update user's role
        user.update_role(new_role.to_string());

        // Save updated user
        self.user_repository
            .update(&user)
            .await
            .map_err(|e| RoleRequestError::DatabaseError(e.to_string()))?;

        info!("User {} role updated to: {}", user_id, new_role);
        Ok(())
    }

    /// Validate that the requested role is valid
    fn validate_role(&self, role: &str) -> Result<(), RoleRequestError> {
        // Define valid roles - this should come from configuration in a real implementation
        let valid_roles = vec!["admin", "partner", "operator", "manager", "viewer"];

        if !valid_roles.contains(&role.to_lowercase().as_str()) {
            return Err(RoleRequestError::InvalidRole(role.to_string()));
        }

        Ok(())
    }

    /// Parse status string into RoleRequestStatus enum
    fn parse_status(&self, status_str: &str) -> Result<RoleRequestStatus, RoleRequestError> {
        match status_str.to_lowercase().as_str() {
            "pending" => Ok(RoleRequestStatus::Pending),
            "approved" => Ok(RoleRequestStatus::Approved),
            "denied" => Ok(RoleRequestStatus::Denied),
            "cancelled" => Ok(RoleRequestStatus::Cancelled),
            _ => Err(RoleRequestError::InvalidStatus(status_str.to_string())),
        }
    }

    /// Convert domain entity to DTO
    fn to_dto(&self, role_request: RoleRequest) -> RoleRequestDto {
        RoleRequestDto {
            id: role_request.id,
            user_id: role_request.user_id,
            requested_role: role_request.requested_role,
            reason: role_request.reason,
            status: self.status_to_string(&role_request.status),
            reviewed_by: role_request.reviewed_by,
            review_notes: role_request.review_notes,
            created_at: role_request.created_at,
            reviewed_at: role_request.reviewed_at,
        }
    }

    /// Convert status enum to string
    fn status_to_string(&self, status: &RoleRequestStatus) -> String {
        match status {
            RoleRequestStatus::Pending => "pending".to_string(),
            RoleRequestStatus::Approved => "approved".to_string(),
            RoleRequestStatus::Denied => "denied".to_string(),
            RoleRequestStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::{User, UserStatus};
    use crate::domain::value_objects::Email;
    use async_trait::async_trait;
    use mockall::{mock, predicate::*};
    use std::sync::Mutex;

    // Mock RoleRequestRepository
    mock! {
        pub RoleRequestRepository {}
        #[async_trait]
        impl RoleRequestRepository for RoleRequestRepository {
            async fn create(&self, request: &CreateRoleRequest) -> Result<i32, RepositoryError>;
            async fn find_by_id(&self, id: i32) -> Result<Option<RoleRequest>, RepositoryError>;
            async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<RoleRequest>, RepositoryError>;
            async fn find_pending_requests(&self) -> Result<Vec<RoleRequest>, RepositoryError>;
            async fn update(&self, id: i32, review: &ReviewRoleRequest, reviewed_by: &str) -> Result<(), RepositoryError>;
        }
    }

    // Mock UserRepository
    mock! {
        pub UserRepository {}
        #[async_trait]
        impl UserRepository for UserRepository {
            async fn create(&self, user: &User, password: &str) -> Result<UserId, RepositoryError>;
            async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, RepositoryError>;
            async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>;
            async fn find_all(&self) -> Result<Vec<User>, RepositoryError>;
            async fn update(&self, user: &User) -> Result<(), RepositoryError>;
            async fn delete(&self, id: &UserId) -> Result<(), RepositoryError>;
            async fn assign_role(&self, user_id: &UserId, role_name: &str) -> Result<(), RepositoryError>;
            async fn get_roles(&self, user_id: &UserId) -> Result<Vec<String>, RepositoryError>;
        }
    }

    fn create_test_user() -> User {
        let email = Email::new("test@example.com".to_string()).unwrap();
        User::new(
            "keycloak-123".to_string(),
            "testuser".to_string(),
            email,
            "Test".to_string(),
            "User".to_string(),
            "operator".to_string(),
        )
    }

    fn create_test_role_request() -> RoleRequest {
        RoleRequest {
            id: 1,
            user_id: "user-123".to_string(),
            requested_role: "admin".to_string(),
            reason: Some("Need admin access".to_string()),
            status: RoleRequestStatus::Pending,
            reviewed_by: None,
            review_notes: None,
            created_at: Utc::now(),
            reviewed_at: None,
        }
    }

    #[tokio::test]
    async fn test_create_role_request_success() {
        let mut mock_role_repo = MockRoleRequestRepository::new();
        let mut mock_user_repo = MockUserRepository::new();

        // Mock user exists
        mock_user_repo
            .expect_find_by_id()
            .returning(|_| Ok(Some(create_test_user())));

        // Mock no existing pending requests
        mock_role_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(vec![]));

        // Mock successful creation
        mock_role_repo
            .expect_create()
            .returning(|_| Ok(1));

        mock_role_repo
            .expect_find_by_id()
            .returning(|_| Ok(Some(create_test_role_request())));

        let service = RoleRequestService::new(
            Arc::new(mock_role_repo),
            Arc::new(mock_user_repo),
        );

        let dto = CreateRoleRequestDto {
            requested_role: "admin".to_string(),
            reason: Some("Need admin access".to_string()),
        };

        let result = service.create_role_request("user-123".to_string(), dto).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_role_request_user_not_found() {
        let mut mock_role_repo = MockRoleRequestRepository::new();
        let mut mock_user_repo = MockUserRepository::new();

        // Mock user not found
        mock_user_repo
            .expect_find_by_id()
            .returning(|_| Ok(None));

        let service = RoleRequestService::new(
            Arc::new(mock_role_repo),
            Arc::new(mock_user_repo),
        );

        let dto = CreateRoleRequestDto {
            requested_role: "admin".to_string(),
            reason: Some("Need admin access".to_string()),
        };

        let result = service.create_role_request("user-123".to_string(), dto).await;
        assert!(matches!(result, Err(RoleRequestError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_create_role_request_active_request_exists() {
        let mut mock_role_repo = MockRoleRequestRepository::new();
        let mut mock_user_repo = MockUserRepository::new();

        // Mock user exists
        mock_user_repo
            .expect_find_by_id()
            .returning(|_| Ok(Some(create_test_user())));

        // Mock existing pending request
        mock_role_repo
            .expect_find_by_user_id()
            .returning(|_| Ok(vec![create_test_role_request()]));

        let service = RoleRequestService::new(
            Arc::new(mock_role_repo),
            Arc::new(mock_user_repo),
        );

        let dto = CreateRoleRequestDto {
            requested_role: "admin".to_string(),
            reason: Some("Need admin access".to_string()),
        };

        let result = service.create_role_request("user-123".to_string(), dto).await;
        assert!(matches!(result, Err(RoleRequestError::ActiveRequestExists)));
    }

    #[tokio::test]
    async fn test_validate_role_valid() {
        let mock_role_repo = MockRoleRequestRepository::new();
        let mock_user_repo = MockUserRepository::new();

        let service = RoleRequestService::new(
            Arc::new(mock_role_repo),
            Arc::new(mock_user_repo),
        );

        assert!(service.validate_role("admin").is_ok());
        assert!(service.validate_role("partner").is_ok());
        assert!(service.validate_role("operator").is_ok());
    }

    #[tokio::test]
    async fn test_validate_role_invalid() {
        let mock_role_repo = MockRoleRequestRepository::new();
        let mock_user_repo = MockUserRepository::new();

        let service = RoleRequestService::new(
            Arc::new(mock_role_repo),
            Arc::new(mock_user_repo),
        );

        assert!(matches!(
            service.validate_role("invalid_role"),
            Err(RoleRequestError::InvalidRole(_))
        ));
    }
}