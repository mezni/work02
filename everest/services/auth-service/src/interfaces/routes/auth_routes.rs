use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use crate::interfaces::controllers::auth_controller::*;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    )
)]
pub async fn register(
    controller: web::Data<AuthController>,
    request: web::Json<RegisterRequest>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.register(request.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    controller: web::Data<AuthController>,
    request: web::Json<LoginRequest>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    let ip_address = req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let user_agent = req.headers().get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    controller.login(request.into_inner(), ip_address, user_agent).await
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
pub async fn refresh_token(
    controller: web::Data<AuthController>,
    request: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.refresh_token(request.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn logout(
    controller: web::Data<AuthController>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.logout(user_id.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found")
    )
)]
pub async fn forgot_password(
    controller: web::Data<AuthController>,
    request: web::Json<ForgotPasswordRequest>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.forgot_password(request.into_inner()).await
}

#[utoipa::path(
    post,
    path = "/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 400, description = "Invalid token or password")
    )
)]
pub async fn reset_password(
    controller: web::Data<AuthController>,
    request: web::Json<ResetPasswordRequest>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.reset_password(request.into_inner()).await
}

#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = 200, description = "User profile retrieved", body = UserProfileDto),
        (status = 401, description = "Not authenticated")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    controller: web::Data<AuthController>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, crate::application::errors::ApplicationError> {
    controller.get_current_user(user_id.into_inner()).await
}

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .route(web::post().to(register))
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(login))
            )
            .service(
                web::resource("/refresh")
                    .route(web::post().to(refresh_token))
            )
            .service(
                web::resource("/logout")
                    .route(web::post().to(logout))
            )
            .service(
                web::resource("/forgot-password")
                    .route(web::post().to(forgot_password))
            )
            .service(
                web::resource("/reset-password")
                    .route(web::post().to(reset_password))
            )
            .service(
                web::resource("/me")
                    .route(web::get().to(get_current_user))
            )
    );
}