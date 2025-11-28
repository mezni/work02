use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use crate::{
    application::{
        dto::{
            LoginRequest, 
            RegisterRequest, 
            AdminCreateUserRequest,
            AuthResponse,
            UserDTO,
        },
        commands::{
            RegisterUserCommand,
            AdminCreateUserCommand,
            LoginUserCommand,
        },
        errors::ApplicationError,
    },
    infrastructure::ioc::ServiceLocator,
};

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful, returning created user", body = UserDTO),
        (status = 400, description = "Invalid input or user exists"),
    )
)]
pub async fn register(
    locator: web::Data<ServiceLocator>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let handler = locator.get_register_handler();
    let cmd = RegisterUserCommand::from(req.0);
    
    let user_dto = handler.execute(cmd).await?;

    Ok(HttpResponse::Created().json(user_dto))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
    )
)]
pub async fn login(
    locator: web::Data<ServiceLocator>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let handler = locator.get_login_handler();
    let cmd = LoginUserCommand::from(req.0);

    let response: AuthResponse = handler.execute(cmd).await?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/admin/create-user",
    request_body = AdminCreateUserRequest,
    responses(
        (status = 201, description = "User creation successful", body = UserDTO),
        (status = 400, description = "Invalid input or user exists"),
        (status = 403, description = "Forbidden (Admin role required)"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_create_user(
    locator: web::Data<ServiceLocator>,
    req: web::Json<AdminCreateUserRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let handler = locator.get_admin_create_user_handler();
    let cmd = AdminCreateUserCommand::from(req.0);

    let user_dto = handler.execute(cmd).await?;

    Ok(HttpResponse::Created().json(user_dto))
}

// Swagger Definition
#[derive(OpenApi)]
#[openapi(
    paths(
        register,
        login,
        admin_create_user,
    ),
    components(
        schemas(
            LoginRequest, 
            RegisterRequest,
            AdminCreateUserRequest, 
            AuthResponse,
            UserDTO
        ),
        security_schemes(
            ("bearer_auth" = (type = "http", scheme = "bearer", bearer_format = "JWT"))
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication and User Management Endpoints")
    )
)]
pub struct ApiDoc;
