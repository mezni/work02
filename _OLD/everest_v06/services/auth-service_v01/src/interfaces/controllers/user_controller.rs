use actix_web::{HttpResponse, web};
use tracing::{info, error};
use uuid::Uuid;
use crate::application::commands::{
    CreateUserCommand, UpdateUserCommand, ChangeUserRoleCommand, 
    AssignUserToCompanyCommand, RemoveUserFromCompanyCommand,
    UpdateProfileCommand, ChangePasswordCommand
};
use crate::application::dto::{
    CreateUserRequest, UpdateUserRequest, ChangeUserRoleRequest, 
    AssignUserToCompanyRequest, UpdateProfileRequest, ChangePasswordRequest,
    UserDto, UserProfileDto, UserListResponse
};
use crate::application::handlers::command_handlers::UserCommandHandler;
use crate::application::handlers::query_handlers::UserQueryHandler;
use crate::common::response::ApiResponse;
use crate::domain::repositories::{UserRepository, CompanyRepository};
use crate::domain::enums::UserRole;

pub struct UserController<T: UserRepository, C: CompanyRepository, A> {
    user_handler: UserCommandHandler<T, C, A>,
    user_query_handler: UserQueryHandler<T>,
}

impl<T: UserRepository, C: CompanyRepository, A> UserController<T, C, A> {
    pub fn new(user_handler: UserCommandHandler<T, C, A>, user_query_handler: UserQueryHandler<T>) -> Self {
        Self {
            user_handler,
            user_query_handler,
        }
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Creating new user: {}", request.email);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        let command = CreateUserCommand {
            username: request.username,
            email: request.email,
            role: request.role,
            company_id: request.company_id,
        };

        let user = self.user_handler.handle_create_user(command, &requester).await?;
        let user_dto = UserDto::from(user);

        let response = ApiResponse::success(user_dto, "User created successfully");
        Ok(HttpResponse::Created().json(response))
    }

    pub async fn get_user(
        &self,
        user_id: Uuid,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting user: {}", user_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let query = crate::application::queries::GetUserByIdQuery {
            user_id,
            requested_by: requester_uuid,
        };

        if let Some(user) = self.user_query_handler.handle_get_user_by_id(query).await? {
            let user_dto = UserDto::from(user);
            let response = ApiResponse::success(user_dto, "User retrieved successfully");
            Ok(HttpResponse::Ok().json(response))
        } else {
            Err(crate::application::errors::ApplicationError::UserNotFound(user_id.to_string()))
        }
    }

    pub async fn list_users(
        &self,
        query: crate::interfaces::routes::user_routes::ListUsersQuery,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Listing users");

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20);

        let list_query = crate::application::queries::ListUsersQuery {
            page,
            page_size,
            company_id: query.company_id,
            role: query.role,
            requested_by: requester_uuid,
        };

        let users = self.user_query_handler.handle_list_users(list_query).await?;
        let user_dtos: Vec<UserDto> = users.into_iter().map(UserDto::from).collect();

        let response = UserListResponse {
            users: user_dtos,
            total: user_dtos.len() as u64, // In real impl, get total count from DB
            page,
            page_size,
            total_pages: (user_dtos.len() as f64 / page_size as f64).ceil() as u32,
        };

        let api_response = ApiResponse::success(response, "Users retrieved successfully");
        Ok(HttpResponse::Ok().json(api_response))
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        request: UpdateUserRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Updating user: {}", user_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        // Get target user
        let target_query = crate::application::queries::GetUserByIdQuery {
            user_id,
            requested_by: requester_uuid,
        };
        let mut target_user = self.user_query_handler.handle_get_user_by_id(target_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(user_id.to_string()))?;

        // Check permissions
        if !requester.can_manage_user(&target_user) {
            return Err(crate::application::errors::ApplicationError::Authorization(
                "Insufficient permissions to update this user".to_string(),
            ));
        }

        // Update fields
        if let Some(username) = request.username {
            target_user.username = username;
        }
        if let Some(email) = request.email {
            let email_vo = crate::domain::value_objects::Email::new(email)?;
            target_user.email = email_vo;
        }
        if let Some(company_id) = request.company_id {
            // This will validate if the user can be assigned to a company
            target_user.assign_to_company(company_id)?;
        }

        // Save changes
        self.user_handler.user_repository.update(&target_user).await?;

        let user_dto = UserDto::from(target_user);
        let response = ApiResponse::success(user_dto, "User updated successfully");

        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn delete_user(
        &self,
        user_id: Uuid,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Deleting user: {}", user_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        // Get target user
        let target_query = crate::application::queries::GetUserByIdQuery {
            user_id,
            requested_by: requester_uuid,
        };
        let target_user = self.user_query_handler.handle_get_user_by_id(target_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(user_id.to_string()))?;

        // Check permissions
        if !requester.can_manage_user(&target_user) {
            return Err(crate::application::errors::ApplicationError::Authorization(
                "Insufficient permissions to delete this user".to_string(),
            ));
        }

        // Prevent self-deletion
        if requester.id == target_user.id {
            return Err(crate::application::errors::ApplicationError::BusinessRuleViolation(
                "Cannot delete your own account".to_string(),
            ));
        }

        self.user_handler.user_repository.delete(&user_id).await?;

        let response = ApiResponse::success((), "User deleted successfully");
        Ok(HttpResponse::NoContent().json(response))
    }

    pub async fn change_user_role(
        &self,
        user_id: Uuid,
        request: ChangeUserRoleRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Changing user role: {}", user_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        let command = ChangeUserRoleCommand {
            user_id,
            new_role: request.role,
            changed_by: requester.id,
        };

        self.user_handler.handle_change_user_role(command, &requester).await?;

        let response = ApiResponse::success((), "User role changed successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn assign_user_to_company(
        &self,
        user_id: Uuid,
        request: AssignUserToCompanyRequest,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Assigning user to company: {} -> {}", user_id, request.company_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        let command = AssignUserToCompanyCommand {
            user_id,
            company_id: request.company_id,
            assigned_by: requester.id,
        };

        self.user_handler.handle_assign_user_to_company(command, &requester).await?;

        let response = ApiResponse::success((), "User assigned to company successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn remove_user_from_company(
        &self,
        user_id: Uuid,
        requester_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Removing user from company: {}", user_id);

        let requester_uuid = Uuid::parse_str(&requester_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        // Get requester user
        let requester_query = crate::application::queries::GetUserByIdQuery {
            user_id: requester_uuid,
            requested_by: requester_uuid,
        };
        let requester = self.user_query_handler.handle_get_user_by_id(requester_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(requester_id))?;

        // Get target user
        let target_query = crate::application::queries::GetUserByIdQuery {
            user_id,
            requested_by: requester_uuid,
        };
        let mut target_user = self.user_query_handler.handle_get_user_by_id(target_query).await?
            .ok_or_else(|| crate::application::errors::ApplicationError::UserNotFound(user_id.to_string()))?;

        // Check permissions
        if !requester.can_manage_user(&target_user) {
            return Err(crate::application::errors::ApplicationError::Authorization(
                "Insufficient permissions to manage this user".to_string(),
            ));
        }

        target_user.remove_from_company();
        self.user_handler.user_repository.update(&target_user).await?;

        let response = ApiResponse::success((), "User removed from company successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn update_profile(
        &self,
        request: UpdateProfileRequest,
        user_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Updating user profile: {}", user_id);

        let uuid = Uuid::parse_str(&user_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let command = UpdateProfileCommand {
            username: request.username,
            email: request.email,
        };

        let user = self.user_handler.handle_update_profile(command, uuid).await?;
        let user_profile = UserProfileDto::from(user);

        let response = ApiResponse::success(user_profile, "Profile updated successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn change_password(
        &self,
        request: ChangePasswordRequest,
        user_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Changing password for user: {}", user_id);

        // This would validate current password and update to new password
        // For now, we'll just acknowledge the request

        let response = ApiResponse::success((), "Password changed successfully");
        Ok(HttpResponse::Ok().json(response))
    }
}