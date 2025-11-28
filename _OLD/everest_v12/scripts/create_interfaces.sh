#!/bin/bash
# create_interfaces_layer_with_stubs.sh
# Creates the Interfaces layer for Auth-Service with Actix-Web and Swagger
SERVICE_DIR="auth-service"

echo "Creating Interfaces layer structure with starter code..."

cd $SERVICE_DIR

# Create directories
mkdir -p src/interfaces/http

# Create mod.rs
cat > src/interfaces/mod.rs <<EOL
pub mod http;
EOL

# HTTP routes stub
cat > src/interfaces/http/routes.rs <<EOL
use actix_web::web;
use crate::application::handlers::{register_handler, login_handler};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register_handler::handle);
    cfg.service(login_handler::handle);
}
EOL

# Middleware stub
cat > src/interfaces/http/middleware.rs <<EOL
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::middleware::{Middleware, Next};
use tracing::info;

pub struct LoggingMiddleware;

impl<S, B> Middleware<S> for LoggingMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    fn call(&self, req: ServiceRequest, next: Next<S>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ServiceResponse<B>, Error>>>> {
        info!("Incoming request: {} {}", req.method(), req.path());
        Box::pin(async move { next.call(req).await })
    }
}
EOL

# Swagger integration stub
cat > src/interfaces/http/swagger.rs <<EOL
use utoipa::OpenApi;
use crate::application::dto::{user_dto::UserDTO, auth_response::AuthResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::application::handlers::register_handler::handle,
        crate::application::handlers::login_handler::handle
    ),
    components(
        schemas(UserDTO, AuthResponse)
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
pub struct ApiDoc;
EOL

echo "✅ Interfaces layer structure and starter code created successfully!"
echo "Directories and files:"
echo "- src/interfaces/mod.rs"
echo "- src/interfaces/http/routes.rs"
echo "- src/interfaces/http/middleware.rs"
echo "- src/interfaces/http/swagger.rs"
