#!/bin/bash

set -e

cargo new auth-service

cd auth-service

# Create directories
mkdir -p src/domain
mkdir -p src/infrastructure
mkdir -p src/application
mkdir -p src/interfaces
mkdir -p src/tests

# Create main.rs
cat > src/main.rs << 'EOF'
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use infrastructure::config::Config;
use infrastructure::db::DBClient;
use interfaces::routes::init_routes;

mod domain;
mod infrastructure;
mod application;
mod interfaces;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: DBClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::init();
    let db = DBClient::init(&config.database_url).await.expect("DB init failed");

    println!("Server running on http://localhost:{}", config.port);

    let app_state = AppState { config, db };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(init_routes)
    })
    .bind(("0.0.0.0", app_state.config.port))?
    .run()
    .await
}
EOF

# Create lib.rs
cat > src/lib.rs << 'EOF'
pub mod domain;
pub mod infrastructure;
pub mod application;
pub mod interfaces;
EOF

# domain/errors.rs
cat > src/domain/errors.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid role")]
    InvalidRole,
}
EOF

# domain/user.rs
cat > src/domain/user.rs << 'EOF'
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use crate::domain::errors::DomainError;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::User => "user",
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub photo: String,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn validate_role(&self) -> Result<(), DomainError> {
        match self.role {
            UserRole::Admin | UserRole::Moderator | UserRole::User => Ok(()),
        }
    }
}
EOF

# infrastructure/config.rs
cat > src/infrastructure/config.rs << 'EOF'
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        dotenv::dotenv().ok();
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
EOF

# infrastructure/db.rs
cat > src/infrastructure/db.rs << 'EOF'
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct DBClient {
    pub pool: PgPool,
}

impl DBClient {
    pub async fn init(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
}
EOF

# infrastructure/user_repository.rs
cat > src/infrastructure/user_repository.rs << 'EOF'
use crate::domain::user::{User, UserRole};
use crate::domain::errors::DomainError;
use crate::infrastructure::db::DBClient;
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(db: &DBClient) -> Self {
        Self { pool: db.pool.clone() }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, DomainError> {
        Err(DomainError::UserNotFound)
    }

    pub async fn create(&self, user: &User) -> Result<User, DomainError> {
        Ok(user.clone())
    }
}
EOF

# application/user_dtos.rs
cat > src/application/user_dtos.rs << 'EOF'
use serde::{Deserialize, Serialize};
use crate::domain::user::{UserRole, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
        }
    }
}
EOF

# application/user_service.rs
cat > src/application/user_service.rs << 'EOF'
use crate::application::user_dtos::{LoginUserDto, RegisterUserDto, UserResponseDto};
use crate::infrastructure::user_repository::UserRepository;
use crate::domain::errors::DomainError;

#[derive(Clone)]
pub struct UserService {
    pub repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn register(&self, _dto: RegisterUserDto) -> Result<UserResponseDto, DomainError> {
        Err(DomainError::UserAlreadyExists)
    }

    pub async fn login(&self, _dto: LoginUserDto) -> Result<String, DomainError> {
        Ok("jwt_token_placeholder".into())
    }
}
EOF

# interfaces/routes.rs
cat > src/interfaces/routes.rs << 'EOF'
use actix_web::web;
pub mod user_handlers;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user_handlers::register_user);
    cfg.service(user_handlers::login_user);
}
EOF

# interfaces/user_handlers.rs
cat > src/interfaces/user_handlers.rs << 'EOF'
use actix_web::{post, web, HttpResponse, Responder};
use crate::application::user_dtos::{RegisterUserDto, LoginUserDto};
use crate::application::user_service::UserService;
use crate::main::AppState;
use crate::infrastructure::user_repository::UserRepository;

#[post("/api/register")]
pub async fn register_user(
    state: web::Data<AppState>,
    dto: web::Json<RegisterUserDto>,
) -> impl Responder {
    let service = UserService::new(UserRepository::new(&state.db));
    match service.register(dto.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[post("/api/login")]
pub async fn login_user(
    state: web::Data<AppState>,
    dto: web::Json<LoginUserDto>,
) -> impl Responder {
    let service = UserService::new(UserRepository::new(&state.db));
    match service.login(dto.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
EOF

# interfaces/openapi.rs
cat > src/interfaces/openapi.rs << 'EOF'
use utoipa::{OpenApi, Modify, openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme}};
use crate::interfaces::user_handlers::{register_user, login_user};

#[derive(OpenApi)]
#[openapi(
    paths(register_user, login_user),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build())
        );
    }
}
EOF

# Make script executable
chmod +x setup.sh

echo "Project scaffold created! Run 'cargo check' to verify it compiles."
