use crate::application::dtos::health::HealthResponse;
use crate::application::dtos::registration::{
    RegisterRequest, RegisterResponse, ResendRequest, ResendResponse, VerifyRequest, VerifyResponse,
};
// Added this import to bring Auth DTOs into scope for the OpenApi macro
use crate::application::dtos::authentication::{
    LoginRequest, LoginResponseDto, LogoutRequest, MessageResponse, RefreshTokenRequest,
    UserInfoDto,
};
use crate::presentation::controllers::{
    authentication_controller, health_controller, registration_controller,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health_controller::health_check,
        registration_controller::register_user,
        registration_controller::verify_registration,
        registration_controller::resend_verification,
        authentication_controller::login,
        authentication_controller::logout,
        authentication_controller::refresh_token,
    ),
    components(
        schemas(
            HealthResponse,
            RegisterRequest,
            RegisterResponse,
            VerifyRequest,
            VerifyResponse,
            ResendRequest,
            ResendResponse,
            LoginRequest,
            LoginResponseDto,
            UserInfoDto,
            RefreshTokenRequest,
            LogoutRequest,
            MessageResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Registration", description = "User registration and verification"),
        (name = "Authentication", description = "User authentication"),
    ),
    info(
        title = "Authentication Service API",
        version = "1.0.0",
        description = "API for user registration, verification, and authentication"
    )
)]
pub struct ApiDoc;
