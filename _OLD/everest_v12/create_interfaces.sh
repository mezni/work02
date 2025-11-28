#!/bin/bash
# create_interfaces_layer_fixed.sh
# Creates the Interfaces layer for Auth-Service with Actix-Web and Swagger

SERVICE_DIR="auth-service"

echo "Applying fixes and enhancements to the Interfaces layer structure..."

cd $SERVICE_DIR

# --- 1. Create/Fix directories (ensuring all are present)
mkdir -p src/interfaces/http

# --- 2. Create mod.rs (Correct)
cat > src/interfaces/mod.rs <<EOL
pub mod http;
EOL

# --- 3. FIX: Create the HTTP Controller to map requests to Application Handlers
cat > src/interfaces/http/auth_controller.rs <<EOL
use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use crate::{
    application::{
        dto::{
            auth_request::{LoginRequest, RegisterRequest, AdminCreateUserRequest},
            auth_response::AuthResponse,
            user_dto::UserDTO,
        },
        commands::{
            register_user::RegisterUserCommand,
            admin_create_user::AdminCreateUserCommand,
        },
        handlers::{
            register_handler::RegisterUserHandler,
            login_handler::LoginHandler,
            admin_create_user_handler::AdminCreateUserHandler,
        },
        errors::ApplicationError,
    },
    infrastructure::ioc::ServiceLocator,
};

// --- HTTP Controller Functions ---
// These functions map Actix-Web types (web::Json, web::Data) to the pure Application Handlers.

#[utoipa::path(
    post,
    path = "/auth/register",
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
    let handler: RegisterUserHandler<_, _> = locator.get_register_handler();
    let cmd = RegisterUserCommand::from(req.0);
    
    let user_dto = handler.execute(cmd).await?;

    Ok(HttpResponse::Created().json(user_dto))
}


#[utoipa::path(
    post,
    path = "/auth/login",
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

    let response: AuthResponse = handler.execute(req.0).await?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/auth/admin/create-user",
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

// FIX: Define the ApiDoc here using the controller functions.
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
        )
    ),
    tags(
        (name = "Auth", description = "Authentication and User Management Endpoints")
    ),
    // Define the security scheme for Admin routes
    components(
        security_schemes(
            ("bearer_auth" = (type = "http", scheme = "bearer", bearer_format = "JWT"))
        )
    )
)]
pub struct ApiDoc;
EOL

# --- 4. Fix http/mod.rs and http/routes.rs to use the Controller
cat > src/interfaces/http/mod.rs <<EOL
pub mod auth_controller;
pub mod routes;
pub mod middleware;
EOL

cat > src/interfaces/http/routes.rs <<EOL
use actix_web::web;
use crate::interfaces::http::auth_controller::{register, login, admin_create_user};

// FIX: Routes now point to the Actix-Web controller functions
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/admin/create-user", web::post().to(admin_create_user))
    );
}
EOL

# --- 5. Fix middleware.rs (Use the simpler actix-web Logger for tracing integration)
cat > src/interfaces/http/middleware.rs <<EOL
// FIX: Actix-web's built-in TracingLogger is superior and preferred for simple logging.
// This file will remain mostly empty or house custom authentication middleware later.
// For now, we stub a custom authorization header checker.

use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    Error, HttpMessage, 
};
use std::{future::{ready, Ready}, pin::Pin};

// Simple stub for an Authorization Guard/Middleware
pub struct AuthGuard;

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardMiddleware { service }))
    }
}

pub struct AuthGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Here, a real implementation would check for "Authorization: Bearer <JWT>"
        // Decode the JWT, validate the signature, and attach the decoded Claims 
        // (including Role) to the request's extensions for the controller to use.
        // For this stub, we just log and pass through.
        tracing::info!("AuthGuard: Checking JWT on request to {}", req.path());
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
EOL
rm -f src/interfaces/http/swagger.rs # Remove the old, incorrectly placed swagger file

echo "✅ Interfaces layer structure fixed, introducing the necessary Controller (auth_controller.rs) to bridge HTTP and Application layers."