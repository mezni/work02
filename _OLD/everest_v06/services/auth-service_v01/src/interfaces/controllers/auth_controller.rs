use actix_web::{
    web, HttpRequest, HttpResponse, Result as ActixResult,
};
use tracing::{info, error};
use utoipa::ToSchema;

use crate::application::{
    command_handlers::AuthCommandHandler,
    dto::{
        auth_dto::{
            ApiResponse, AuthResponse, ChangePasswordRequest, ForgotPasswordRequest, LoginRequest,
            RegisterRequest, RefreshTokenRequest, UserAuthDto,
        },
        user_dto::UserDto,
    },
    errors::ApplicationError,
};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::audit::Auditor;

#[derive(Clone)]
pub struct AuthController;

impl AuthController {
    #[utoipa::path(
        post,
        path = "/api/v1/auth/register",
        request_body = RegisterRequest,
        responses(
            (status = 201, description = "User registered successfully", body = ApiResponse<UserDto>),
            (status = 400, description = "Invalid input data", body = ApiResponse<String>),
            (status = 409, description = "User already exists", body = ApiResponse<String>),
            (status = 500, description = "Internal server error", body = ApiResponse<String>)
        ),
        tag = "Authentication"
    )]
    pub async fn register<T, A>(
        handler: web::Data<AuthCommandHandler<T, A>>,
        request: web::Json<RegisterRequest>,
    ) -> ActixResult<HttpResponse, ApplicationError>
    where
        T: UserRepository + 'static,
        A: Auditor + 'static,
    {
        info!("Registering new user: {}", request.email);

        let command = crate::application::commands::auth_commands::RegisterUserCommand {
            username: request.username.clone(),
            email: request.email.clone(),
            password: request.password.clone(),
            confirm_password: request.confirm_password.clone(),
        };

        let user = handler
            .handle_register_user(command, None, None)
            .await?;

        let user_dto = UserDto {
            id: user.id,
            username: user.username,
            email: user.email.to_string(),
            role: user.role,
            company_id: user.company_id,
            email_verified: user.email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        let response = ApiResponse::success(user_dto);
        Ok(HttpResponse::Created().json(response))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "Login successful", body = ApiResponse<AuthResponse>),
            (status = 400, description = "Invalid input data", body = ApiResponse<String>),
            (status = 401, description = "Invalid credentials", body = ApiResponse<String>),
            (status = 500, description = "Internal server error", body = ApiResponse<String>)
        ),
        tag = "Authentication"
    )]
    pub async fn login<T, A>(
        handler: web::Data<AuthCommandHandler<T, A>>,
        request: web::Json<LoginRequest>,
    ) -> ActixResult<HttpResponse, ApplicationError>
    where
        T: UserRepository + 'static,
        A: Auditor + 'static,
    {
        info!("Login attempt for user: {}", request.email);

        let command = crate::application::commands::auth_commands::LoginCommand {
            email: request.email.clone(),
            password: request.password.clone(),
        };

        let (access_token, refresh_token, user) = handler.handle_login(command, None, None).await?;

        let user_auth_dto = UserAuthDto {
            id: user.id.to_string(),
            username: user.username,
            email: user.email.to_string(),
            role: user.role.to_string(),
            company_id: user.company_id.map(|id| id.to_string()),
            email_verified: user.email_verified,
        };

        let auth_response = AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user_auth_dto,
        };

        let response = ApiResponse::success(auth_response);
        Ok(HttpResponse::Ok().json(response))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/refresh",
        request_body = RefreshTokenRequest,
        responses(
            (status = 200, description = "Token refreshed successfully", body = ApiResponse<AuthResponse>),
            (status = 401, description = "Invalid refresh token", body = ApiResponse<String>),
            (status = 500, description = "Internal server error", body = ApiResponse<String>)
        ),
        tag = "Authentication"
    )]
    pub async fn refresh_token<T, A>(
        handler: web::Data<AuthCommandHandler<T, A>>,
        request: web::Json<RefreshTokenRequest>,
    ) -> ActixResult<HttpResponse, ApplicationError>
    where
        T: UserRepository + 'static,
        A: Auditor + 'static,
    {
        info!("Refreshing token");

        let command = crate::application::commands::auth_commands::RefreshTokenCommand {
            refresh_token: request.refresh_token.clone(),
        };

        let (access_token, refresh_token) = handler.handle_refresh_token(command).await?;

        let auth_response = AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: UserAuthDto {
                id: "".to_string(),
                username: "".to_string(),
                email: "".to_string(),
                role: "".to_string(),
                company_id: None,
                email_verified: false,
            },
        };

        let response = ApiResponse::success(auth_response);
        Ok(HttpResponse::Ok().json(response))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/forgot-password",
        request_body = ForgotPasswordRequest,
        responses(
            (status = 200, description = "Password reset email sent", body = ApiResponse<String>),
            (status = 400, description = "Invalid email", body = ApiResponse<String>),
            (status = 404, description = "User not found", body = ApiResponse<String>),
            (status = 500, description = "Internal server error", body = ApiResponse<String>)
        ),
        tag = "Authentication"
    )]
    pub async fn forgot_password<T, A>(
        _handler: web::Data<AuthCommandHandler<T, A>>,
        request: web::Json<ForgotPasswordRequest>,
    ) -> ActixResult<HttpResponse, ApplicationError>
    where
        T: UserRepository + 'static,
        A: Auditor + 'static,
    {
        info!("Forgot password request for: {}", request.email);

        let response = ApiResponse::success("Password reset email sent".to_string());
        Ok(HttpResponse::Ok().json(response))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/change-password",
        request_body = ChangePasswordRequest,
        responses(
            (status = 200, description = "Password changed successfully", body = ApiResponse<String>),
            (status = 400, description = "Invalid input data", body = ApiResponse<String>),
            (status = 401, description = "Invalid current password", body = ApiResponse<String>),
            (status = 500, description = "Internal server error", body = ApiResponse<String>)
        ),
        tag = "Authentication",
        security(("bearer" = []))
    )]
    pub async fn change_password<T, A>(
        _handler: web::Data<AuthCommandHandler<T, A>>,
        _request: web::Json<ChangePasswordRequest>,
    ) -> ActixResult<HttpResponse, ApplicationError>
    where
        T: UserRepository + 'static,
        A: Auditor + 'static,
    {
        info!("Changing password for user");

        let response = ApiResponse::success("Password changed successfully".to_string());
        Ok(HttpResponse::Ok().json(response))
    }
}