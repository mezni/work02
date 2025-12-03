use actix_web::{web, HttpResponse, HttpRequest};
use utoipa::{ToSchema, IntoParams};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    application::{
        AuthService, RegistrationService, TokenService,
        service_traits::{AuthServiceTrait, RegistrationServiceTrait, TokenServiceTrait, UserServiceTrait},
        commands::*,
        queries::*,
        error::ApplicationError,
    },
    domain::{User, Token},
    interfaces::{
        dtos::*,
        error::InterfaceError,
        middleware::AuthMiddleware,
    },
    infrastructure::health_check::HealthChecker,
};

// Handler Traits
pub trait AuthHandler: Send + Sync {
    async fn login(
        &self,
        command: web::Json<LoginCommand>,
        req: HttpRequest,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn register(
        &self,
        command: web::Json<RegisterCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn refresh_token(
        &self,
        command: web::Json<RefreshTokenCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn logout(
        &self,
        command: web::Json<LogoutCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn verify_email(
        &self,
        command: web::Json<VerifyEmailCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn resend_verification(
        &self,
        command: web::Json<ResendVerificationCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn reset_password(
        &self,
        command: web::Json<ResetPasswordCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn confirm_password_reset(
        &self,
        command: web::Json<ConfirmPasswordResetCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn validate_token(
        &self,
        command: web::Json<ValidateTokenQuery>,
    ) -> Result<HttpResponse, InterfaceError>;
}

pub trait UserHandler: Send + Sync {
    async fn get_current_user(
        &self,
        user: User,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn update_current_user(
        &self,
        user: User,
        command: web::Json<UpdateProfileCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn change_password(
        &self,
        user: User,
        command: web::Json<ChangePasswordCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
}

pub trait HealthHandler: Send + Sync {
    async fn health_check(&self) -> Result<HttpResponse, InterfaceError>;
    async fn detailed_health_check(&self) -> Result<HttpResponse, InterfaceError>;
}

pub trait AdminHandler: Send + Sync {
    async fn list_users(
        &self,
        query: web::Query<ListUsersQuery>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn get_user(
        &self,
        user_id: web::Path<uuid::Uuid>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn update_user_role(
        &self,
        user_id: web::Path<uuid::Uuid>,
        command: web::Json<UpdateUserRoleCommand>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn activate_user(
        &self,
        user_id: web::Path<uuid::Uuid>,
    ) -> Result<HttpResponse, InterfaceError>;
    
    async fn deactivate_user(
        &self,
        user_id: web::Path<uuid::Uuid>,
    ) -> Result<HttpResponse, InterfaceError>;
}

// Concrete Implementations

#[derive(Clone)]
pub struct AuthHandlerImpl {
    auth_service: Arc<dyn AuthServiceTrait>,
    registration_service: Arc<dyn RegistrationServiceTrait>,
    token_service: Arc<dyn TokenServiceTrait>,
}

impl AuthHandlerImpl {
    pub fn new(
        auth_service: Arc<dyn AuthServiceTrait>,
        registration_service: Arc<dyn RegistrationServiceTrait>,
        token_service: Arc<dyn TokenServiceTrait>,
    ) -> Self {
        Self {
            auth_service,
            registration_service,
            token_service,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginCommand,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Invalid credentials"),
    )
)]
#[tracing::instrument(skip(self, command, req))]
impl AuthHandler for AuthHandlerImpl {
    async fn login(
        &self,
        command: web::Json<LoginCommand>,
        req: HttpRequest,
    ) -> Result<HttpResponse, InterfaceError> {
        // Validate command
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        // Extract IP and User-Agent
        let ip_address = req.connection_info().peer_addr().map(|s| s.to_string());
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        // Call service
        let token = self.auth_service.login(&command.email, &command.password).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        // Convert to response DTO
        let response = AuthResponse::from_token(token);
        
        Ok(HttpResponse::Ok().json(response))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/register",
        request_body = RegisterCommand,
        responses(
            (status = 201, description = "Registration successful", body = RegistrationResponse),
            (status = 400, description = "Invalid request"),
            (status = 409, description = "Email already exists"),
        )
    )]
    async fn register(
        &self,
        command: web::Json<RegisterCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let user_id = self.registration_service.register(&command.email, &command.password).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let response = RegistrationResponse {
            message: "Registration successful. Please verify your email.".to_string(),
            user_id,
        };
        
        Ok(HttpResponse::Created().json(response))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/refresh",
        request_body = RefreshTokenCommand,
        responses(
            (status = 200, description = "Token refreshed", body = AuthResponse),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Invalid token"),
        )
    )]
    async fn refresh_token(
        &self,
        command: web::Json<RefreshTokenCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let token = self.token_service.refresh_token(&command.refresh_token).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let response = AuthResponse::from_token(token);
        Ok(HttpResponse::Ok().json(response))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/logout",
        request_body = LogoutCommand,
        responses(
            (status = 200, description = "Logout successful"),
            (status = 400, description = "Invalid request"),
        )
    )]
    async fn logout(
        &self,
        command: web::Json<LogoutCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        self.auth_service.logout(&command.refresh_token).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Successfully logged out"
        })))
    }
    
    async fn verify_email(
        &self,
        command: web::Json<VerifyEmailCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        self.registration_service.verify_email(&command.token).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Email verified successfully"
        })))
    }
    
    async fn resend_verification(
        &self,
        command: web::Json<ResendVerificationCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        self.registration_service.resend_verification(&command.email).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Verification email sent"
        })))
    }
    
    async fn reset_password(
        &self,
        command: web::Json<ResetPasswordCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        self.auth_service.reset_password(&command.email).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Password reset instructions sent"
        })))
    }
    
    async fn confirm_password_reset(
        &self,
        command: web::Json<ConfirmPasswordResetCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        self.auth_service.confirm_password_reset(&command.token, &command.new_password).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Password reset successful"
        })))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/auth/validate",
        request_body = ValidateTokenQuery,
        responses(
            (status = 200, description = "Token valid", body = UserResponse),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Invalid token"),
        )
    )]
    async fn validate_token(
        &self,
        command: web::Json<ValidateTokenQuery>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_query()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let user = self.token_service.validate_token(&command.token).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let response = UserResponse::from_user(user);
        Ok(HttpResponse::Ok().json(response))
    }
}

#[derive(Clone)]
pub struct UserHandlerImpl {
    user_service: Arc<dyn UserServiceTrait>,
    token_service: Arc<dyn TokenServiceTrait>,
}

impl UserHandlerImpl {
    pub fn new(
        user_service: Arc<dyn UserServiceTrait>,
        token_service: Arc<dyn TokenServiceTrait>,
    ) -> Self {
        Self {
            user_service,
            token_service,
        }
    }
}

impl UserHandler for UserHandlerImpl {
    #[utoipa::path(
        get,
        path = "/api/v1/users/me",
        responses(
            (status = 200, description = "Current user", body = UserResponse),
            (status = 401, description = "Unauthorized"),
        )
    )]
    async fn get_current_user(
        &self,
        user: User,
    ) -> Result<HttpResponse, InterfaceError> {
        let response = UserResponse::from_user(user);
        Ok(HttpResponse::Ok().json(response))
    }
    
    #[utoipa::path(
        patch,
        path = "/api/v1/users/me",
        request_body = UpdateProfileCommand,
        responses(
            (status = 200, description = "Profile updated", body = UserResponse),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Unauthorized"),
        )
    )]
    async fn update_current_user(
        &self,
        user: User,
        command: web::Json<UpdateProfileCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let updated_user = self.user_service.update_user_profile(
            user.id,
            command.company_name.clone(),
            command.station_name.clone(),
        ).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let response = UserResponse::from_user(updated_user);
        Ok(HttpResponse::Ok().json(response))
    }
    
    #[utoipa::path(
        post,
        path = "/api/v1/users/me/password",
        request_body = ChangePasswordCommand,
        responses(
            (status = 200, description = "Password changed"),
            (status = 400, description = "Invalid request"),
            (status = 401, description = "Unauthorized"),
        )
    )]
    async fn change_password(
        &self,
        user: User,
        command: web::Json<ChangePasswordCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        command.validate_command()
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        self.user_service.change_password(
            user.id,
            &command.current_password,
            &command.new_password,
        ).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Password changed successfully"
        })))
    }
}

#[derive(Clone)]
pub struct HealthHandlerImpl {
    health_checker: Arc<HealthChecker>,
}

impl HealthHandlerImpl {
    pub fn new(health_checker: Arc<HealthChecker>) -> Self {
        Self { health_checker }
    }
}

impl HealthHandler for HealthHandlerImpl {
    #[utoipa::path(
        get,
        path = "/api/v1/health",
        responses(
            (status = 200, description = "Service health status", body = HealthResponse),
        )
    )]
    async fn health_check(&self) -> Result<HttpResponse, InterfaceError> {
        let is_healthy = self.health_checker.is_healthy().await;
        
        let response = HealthResponse {
            status: if is_healthy { "healthy" } else { "unhealthy" }.to_string(),
            service: "auth-service".to_string(),
            timestamp: chrono::Utc::now(),
        };
        
        Ok(HttpResponse::Ok().json(response))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/health/detailed",
        responses(
            (status = 200, description = "Detailed health status", body = DetailedHealthResponse),
        )
    )]
    async fn detailed_health_check(&self) -> Result<HttpResponse, InterfaceError> {
        let summary = self.health_checker.get_health_summary().await;
        
        let response = DetailedHealthResponse {
            status: match summary.status {
                crate::infrastructure::health_check::HealthState::Healthy => "healthy".to_string(),
                crate::infrastructure::health_check::HealthState::Degraded => "degraded".to_string(),
                crate::infrastructure::health_check::HealthState::Unhealthy => "unhealthy".to_string(),
            },
            services: summary.services,
            total_services: summary.total_services,
            healthy_services: summary.healthy_services,
            timestamp: chrono::Utc::now(),
        };
        
        Ok(HttpResponse::Ok().json(response))
    }
}

#[derive(Clone)]
pub struct AdminHandlerImpl {
    user_service: Arc<dyn UserServiceTrait>,
}

impl AdminHandlerImpl {
    pub fn new(user_service: Arc<dyn UserServiceTrait>) -> Self {
        Self { user_service }
    }
}

impl AdminHandler for AdminHandlerImpl {
    #[utoipa::path(
        get,
        path = "/api/v1/admin/users",
        params(
            ("page" = Option<u32>, Query, description = "Page number"),
            ("limit" = Option<u32>, Query, description = "Items per page"),
            ("role" = Option<String>, Query, description = "Filter by role"),
            ("active" = Option<bool>, Query, description = "Filter by active status"),
        ),
        responses(
            (status = 200, description = "List of users", body = UserListResponse),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden - Admin only"),
        )
    )]
    async fn list_users(
        &self,
        query: web::Query<ListUsersQuery>,
    ) -> Result<HttpResponse, InterfaceError> {
        // This would query the repository
        // For now, return empty list
        let response = UserListResponse {
            users: vec![],
            total: 0,
            page: query.get_page(),
            limit: query.get_limit(),
            total_pages: 0,
        };
        
        Ok(HttpResponse::Ok().json(response))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/admin/users/{user_id}",
        params(
            ("user_id" = uuid::Uuid, Path, description = "User ID"),
        ),
        responses(
            (status = 200, description = "User details", body = UserResponse),
            (status = 401, description = "Unauthorized"),
            (status = 403, description = "Forbidden - Admin only"),
            (status = 404, description = "User not found"),
        )
    )]
    async fn get_user(
        &self,
        user_id: web::Path<uuid::Uuid>,
    ) -> Result<HttpResponse, InterfaceError> {
        let user = self.user_service.get_user(*user_id).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let response = UserResponse::from_user(user);
        Ok(HttpResponse::Ok().json(response))
    }
    
    async fn update_user_role(
        &self,
        user_id: web::Path<uuid::Uuid>,
        command: web::Json<UpdateUserRoleCommand>,
    ) -> Result<HttpResponse, InterfaceError> {
        let role = crate::domain::value_objects::UserRole::from_str(&command.role)
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let user = self.user_service.update_user_role(*user_id, role).await
            .map_err(|e| InterfaceError::InvalidRequest(e.to_string()))?;
        
        let response = UserResponse::from_user(user);
        Ok(HttpResponse::Ok().json(response))
    }
    
    async fn activate_user(
        &self,
        user_id: web::Path<uuid::Uuid>,
    ) -> Result<HttpResponse, InterfaceError> {
        // Implementation would activate user
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User activated"
        })))
    }
    
    async fn deactivate_user(
        &self,
        user_id: web::Path<uuid::Uuid>,
    ) -> Result<HttpResponse, InterfaceError> {
        // Implementation would deactivate user
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "User deactivated"
        })))
    }
}