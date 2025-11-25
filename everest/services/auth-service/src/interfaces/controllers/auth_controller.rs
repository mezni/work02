use actix_web::{HttpResponse, web};
use tracing::{info, error};
use uuid::Uuid;
use crate::application::commands::{
    RegisterUserCommand, LoginCommand, RefreshTokenCommand, ForgotPasswordCommand, 
    ResetPasswordCommand, LogoutCommand
};
use crate::application::dto::{
    RegisterRequest, LoginRequest, RefreshTokenRequest, ForgotPasswordRequest,
    ResetPasswordRequest, AuthResponse, UserAuthDto, UserProfileDto
};
use crate::application::handlers::command_handlers::AuthCommandHandler;
use crate::application::handlers::query_handlers::UserQueryHandler;
use crate::common::response::ApiResponse;
use crate::domain::repositories::UserRepository;

pub struct AuthController<T: UserRepository, A> {
    auth_handler: AuthCommandHandler<T, A>,
    user_query_handler: UserQueryHandler<T>,
}

impl<T: UserRepository, A> AuthController<T, A> {
    pub fn new(auth_handler: AuthCommandHandler<T, A>, user_query_handler: UserQueryHandler<T>) -> Self {
        Self {
            auth_handler,
            user_query_handler,
        }
    }

    pub async fn register(
        &self,
        request: RegisterRequest,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Registering new user: {}", request.email);

        let command = RegisterUserCommand {
            username: request.username,
            email: request.email,
            password: request.password,
            confirm_password: request.confirm_password,
        };

        let user = self.auth_handler.handle_register_user(command, None, None).await?;

        let response = ApiResponse::success(
            UserAuthDto {
                id: user.id.to_string(),
                username: user.username,
                email: user.email.into(),
                role: user.role.to_string(),
                company_id: user.company_id.map(|id| id.to_string()),
                email_verified: user.email_verified,
            },
            "User registered successfully",
        );

        Ok(HttpResponse::Created().json(response))
    }

    pub async fn login(
        &self,
        request: LoginRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Login attempt for user: {}", request.email);

        let command = LoginCommand {
            email: request.email,
            password: request.password,
        };

        let (jwt_token, refresh_token, user) = 
            self.auth_handler.handle_login(command, ip_address, user_agent).await?;

        let auth_response = AuthResponse {
            access_token: jwt_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600, // 1 hour
            user: UserAuthDto {
                id: user.id.to_string(),
                username: user.username,
                email: user.email.into(),
                role: user.role.to_string(),
                company_id: user.company_id.map(|id| id.to_string()),
                email_verified: user.email_verified,
            },
        };

        let response = ApiResponse::success(auth_response, "Login successful");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Refreshing token");

        let command = RefreshTokenCommand {
            refresh_token: request.refresh_token,
        };

        let (jwt_token, refresh_token) = self.auth_handler.handle_refresh_token(command).await?;

        let auth_response = AuthResponse {
            access_token: jwt_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: UserAuthDto { // Placeholder - you'd need to get user info
                id: "".to_string(),
                username: "".to_string(),
                email: "".to_string(),
                role: "".to_string(),
                company_id: None,
                email_verified: false,
            },
        };

        let response = ApiResponse::success(auth_response, "Token refreshed successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn logout(
        &self,
        user_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Logout for user: {}", user_id);

        // In a real implementation, you might blacklist the token
        // or perform other cleanup actions

        let response = ApiResponse::success((), "Logout successful");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn forgot_password(
        &self,
        request: ForgotPasswordRequest,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Forgot password request for: {}", request.email);

        // This would typically send a password reset email
        // For now, we'll just acknowledge the request

        let response = ApiResponse::success((), "If the email exists, a reset link has been sent");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn reset_password(
        &self,
        request: ResetPasswordRequest,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Password reset request");

        // This would validate the reset token and update the password
        // For now, we'll just acknowledge the request

        let response = ApiResponse::success((), "Password reset successfully");
        Ok(HttpResponse::Ok().json(response))
    }

    pub async fn get_current_user(
        &self,
        user_id: String,
    ) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
        info!("Getting current user profile: {}", user_id);

        let uuid = Uuid::parse_str(&user_id)
            .map_err(|e| crate::application::errors::ApplicationError::InvalidInput(e.to_string()))?;

        let query = crate::application::queries::GetUserProfileQuery { user_id: uuid };
        let user = self.user_query_handler.handle_get_user_profile(query).await?;

        let user_profile = UserProfileDto::from(user);
        let response = ApiResponse::success(user_profile, "User profile retrieved successfully");

        Ok(HttpResponse::Ok().json(response))
    }
}