// src/application/services/user_application_service.rs
use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::entities::User;
use crate::domain::value_objects::{Email, Username};
use crate::domain::repositories::UserRepository;
use crate::application::commands::{CreateUserCommand, UpdateUserCommand, DeactivateUserCommand};
use crate::application::queries::{GetUsersQuery, GetUserByIdQuery, SearchUsersQuery};
use crate::application::dtos::{UserDto, UserListDto};
use crate::application::ApplicationError;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, command: CreateUserCommand) -> Result<UserDto, ApplicationError>;
    async fn update_user(&self, user_id: String, command: UpdateUserCommand) -> Result<UserDto, ApplicationError>;
    async fn deactivate_user(&self, user_id: String, command: DeactivateUserCommand) -> Result<UserDto, ApplicationError>;
    async fn get_user_by_id(&self, query: GetUserByIdQuery) -> Result<UserDto, ApplicationError>;
    async fn get_users(&self, query: GetUsersQuery) -> Result<UserListDto, ApplicationError>;
    async fn search_users(&self, query: SearchUsersQuery) -> Result<UserListDto, ApplicationError>;
}

pub struct UserApplicationService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserApplicationService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserApplicationService {
    async fn create_user(&self, command: CreateUserCommand) -> Result<UserDto, ApplicationError> {
        // Validate command
        command.validate()
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        // Check if user already exists
        let email = Email::new(&command.email)
            .map_err(ApplicationError::Domain)?;
            
        if self.user_repository.find_by_email(&email).await?
            .is_some() {
            return Err(ApplicationError::UserAlreadyExists);
        }
        
        let username = Username::new(&command.username)
            .map_err(ApplicationError::Domain)?;
            
        if self.user_repository.find_by_username(&username).await?
            .is_some() {
            return Err(ApplicationError::Validation("Username already exists".to_string()));
        }
        
        // Create user
        let (user, _event) = User::create(
            command.keycloak_id,
            username,
            email,
            command.first_name,
            command.last_name,
        ).map_err(ApplicationError::Domain)?;
        
        // Save user
        self.user_repository.save(&user).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;
        
        // Return DTO
        Ok(UserDto::from(user))
    }
    
    async fn update_user(&self, user_id: String, command: UpdateUserCommand) -> Result<UserDto, ApplicationError> {
        // Validate command
        command.validate()
            .map_err(|e| ApplicationError::Validation(e.to_string()))?;
        
        // Find user
        let user_id = crate::domain::value_objects::UserId::new(user_id)
            .map_err(ApplicationError::Domain)?;
            
        let mut user = self.user_repository.find_by_id(&user_id).await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        // Update user profile
        let _event = user.update_profile(
            command.first_name,
            command.last_name,
            command.phone_number,
        ).map_err(ApplicationError::Domain)?;
        
        // Save updated user
        self.user_repository.save(&user).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;
        
        Ok(UserDto::from(user))
    }
    
    async fn deactivate_user(&self, user_id: String, command: DeactivateUserCommand) -> Result<UserDto, ApplicationError> {
        // Find user
        let user_id = crate::domain::value_objects::UserId::new(user_id)
            .map_err(ApplicationError::Domain)?;
            
        let mut user = self.user_repository.find_by_id(&user_id).await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        // Check if user is trying to deactivate themselves
        if user.id().as_str() == &command.deactivated_by {
            return Err(ApplicationError::Validation("Users cannot deactivate themselves".to_string()));
        }
        
        // Deactivate user
        let _event = user.deactivate()
            .map_err(ApplicationError::Domain)?;
        
        // Save deactivated user
        self.user_repository.save(&user).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;
        
        Ok(UserDto::from(user))
    }
    
    async fn get_user_by_id(&self, query: GetUserByIdQuery) -> Result<UserDto, ApplicationError> {
        let user_id = crate::domain::value_objects::UserId::new(query.user_id)
            .map_err(ApplicationError::Domain)?;
            
        let user = self.user_repository.find_by_id(&user_id).await?
            .ok_or(ApplicationError::UserNotFound)?;
        
        Ok(UserDto::from(user))
    }
    
    async fn get_users(&self, query: GetUsersQuery) -> Result<UserListDto, ApplicationError> {
        let users = self.user_repository.list_users(
            query.page,
            query.page_size,
            query.active_only,
        ).await
        .map_err(|e| ApplicationError::Repository(e.to_string()))?;
        
        // In a real implementation, you'd get total_count from the repository
        let total_count = users.len() as u64;
        
        let user_dtos = users.into_iter()
            .map(UserDto::from)
            .collect();
        
        Ok(UserListDto::new(
            user_dtos,
            query.page,
            query.page_size,
            total_count,
        ))
    }
    
    async fn search_users(&self, query: SearchUsersQuery) -> Result<UserListDto, ApplicationError> {
        // For now, return empty result - implement search logic in repository
        // This would be implemented when you add search capabilities to your repository
        Ok(UserListDto::empty(query.page, query.page_size))
    }
}

// Helper trait for repository error conversion
trait RepositoryResultExt<T> {
    fn into_app_result(self) -> Result<T, ApplicationError>;
}

impl<T> RepositoryResultExt<T> for Result<T, crate::domain::DomainError> {
    fn into_app_result(self) -> Result<T, ApplicationError> {
        self.map_err(|e| match e {
            crate::domain::DomainError::UserNotFound => ApplicationError::UserNotFound,
            crate::domain::DomainError::UserAlreadyExists => ApplicationError::UserAlreadyExists,
            _ => ApplicationError::Repository(e.to_string()),
        })
    }
}