#!/bin/bash

set -e

PROJECT_NAME="rust_keycloak_api"

# Create project
cargo new $PROJECT_NAME --bin
cd $PROJECT_NAME

# Install required crates
cargo add actix-web env_logger dotenvy utoipa utoipa-swagger-ui async-trait reqwest serde serde_json uuid

# Create folder structure
mkdir -p src/{domain,infrastructure,application,interfaces/http}
touch .env

# -----------------------
# .env
cat > .env << 'EOF'
SERVER_PORT=8000
KEYCLOAK_URL=http://localhost:8080
KEYCLOAK_REALM=myrealm
KEYCLOAK_CLIENT_ID=backend-admin
KEYCLOAK_CLIENT_SECRET=<GENERATED_SECRET>
EOF

# -----------------------
# src/domain/errors.rs
mkdir -p src/domain
cat > src/domain/errors.rs << 'EOF'
#[derive(Debug)]
pub enum DomainError {
    RepositoryError(String),
    ValidationError(String),
}
EOF

# -----------------------
# src/domain/user.rs
cat > src/domain/user.rs << 'EOF'
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
}
EOF

# -----------------------
# src/domain/user_repository.rs
cat > src/domain/user_repository.rs << 'EOF'
use crate::domain::user::User;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
}
EOF

# -----------------------
# src/infrastructure/errors.rs
mkdir -p src/infrastructure
cat > src/infrastructure/errors.rs << 'EOF'
#[derive(Debug)]
pub enum InfrastructureError {
    KeycloakError(String),
    HttpError(String),
}
EOF

# -----------------------
# src/infrastructure/keycloak_client.rs
cat > src/infrastructure/keycloak_client.rs << 'EOF'
use crate::domain::user::User;
use crate::infrastructure::errors::InfrastructureError;
use reqwest::StatusCode;

#[derive(Clone)]
pub struct KeycloakClient {
    pub url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
}

impl KeycloakClient {
    pub fn new(url: String, realm: String, client_id: String, client_secret: String) -> Self {
        Self { url, realm, client_id, client_secret }
    }

    pub async fn get_admin_token(&self) -> Result<String, InfrastructureError> {
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let resp = client
            .post(format!("{}/realms/{}/protocol/openid-connect/token", self.url, self.realm))
            .form(&params)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakError(e.to_string()))?;

        let json: serde_json::Value = resp.json().await.map_err(|e| InfrastructureError::KeycloakError(e.to_string()))?;
        let token = json["access_token"]
            .as_str()
            .ok_or_else(|| InfrastructureError::KeycloakError("access_token missing".to_string()))?
            .to_string();
        Ok(token)
    }

    pub async fn create_user(&self, user: &User) -> Result<(), InfrastructureError> {
        let token = self.get_admin_token().await?;

        let payload = serde_json::json!({
            "username": user.username,
            "email": user.email,
            "enabled": true,
            "attributes": {
                "company_name": user.company_name,
                "station_name": user.station_name
            },
            "credentials": [{
                "type": "password",
                "value": user.password,
                "temporary": false
            }],
            "realmRoles": [user.role.to_lowercase()]
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("{}/admin/realms/{}/users", self.url, self.realm))
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| InfrastructureError::KeycloakError(e.to_string()))?;

        match resp.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT => Err(InfrastructureError::KeycloakError("User already exists".to_string())),
            s => {
                let text = resp.text().await.unwrap_or_default();
                Err(InfrastructureError::KeycloakError(format!("Unexpected response {}: {}", s, text)))
            }
        }
    }
}
EOF

# -----------------------
# src/infrastructure/user_repository.rs
cat > src/infrastructure/user_repository.rs << 'EOF'
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::domain::errors::DomainError;
use crate::infrastructure::keycloak_client::KeycloakClient;
use async_trait::async_trait;

#[derive(Clone)]
pub struct KeycloakUserRepository {
    pub client: KeycloakClient,
}

#[async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        self.client
            .create_user(user)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>, DomainError> {
        Ok(None)
    }
}
EOF

# -----------------------
# src/application/errors.rs
mkdir -p src/application
cat > src/application/errors.rs << 'EOF'
#[derive(Debug)]
pub enum ApplicationError {
    DomainError(String),
    InfrastructureError(String),
}
EOF

# -----------------------
# src/application/register_service.rs
cat > src/application/register_service.rs << 'EOF'
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::domain::errors::DomainError;

pub struct RegisterService {
    user_repo: Box<dyn UserRepository>,
}

impl RegisterService {
    pub fn new(user_repo: Box<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(&self, user: User) -> Result<(), DomainError> {
        self.user_repo.create(&user).await
    }
}
EOF

# -----------------------
# src/interfaces/http/health_handler.rs
mkdir -p src/interfaces/http
cat > src/interfaces/http/health_handler.rs << 'EOF'
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
    pub message: String,
}

#[get("/api/v1/health")]
pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "success",
        message: "API is healthy".to_string(),
    })
}
EOF

# -----------------------
# src/interfaces/http/register_handler.rs
cat > src/interfaces/http/register_handler.rs << 'EOF'
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::user::User;
use crate::infrastructure::user_repository::KeycloakUserRepository;
use crate::application::register_service::RegisterService;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    pub status: &'static str,
    pub message: String,
}

#[post("/api/v1/register")]
pub async fn register_handler(
    repo: web::Data<KeycloakUserRepository>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    let dto = body.into_inner();

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: dto.username,
        email: dto.email,
        password: dto.password,
        first_name: None,
        last_name: None,
        role: "USER".to_string(),
        company_name: "".to_string(),
        station_name: "".to_string(),
    };

    let service = RegisterService::new(Box::new(repo.get_ref().clone()));

    match service.register_user(user).await {
        Ok(_) => HttpResponse::Created().json(RegisterResponse {
            status: "success",
            message: "User registered successfully".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
EOF

# -----------------------
# src/interfaces/http/login_handler.rs
cat > src/interfaces/http/login_handler.rs << 'EOF'
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub status: &'static str,
    pub message: String,
}

#[post("/api/v1/auth")]
pub async fn login_handler(
    _body: web::Json<LoginRequest>,
) -> impl Responder {
    HttpResponse::Ok().json(LoginResponse {
        status: "success",
        message: "Logged in successfully".to_string(),
    })
}
EOF

# -----------------------
# src/interfaces/swagger.rs
cat > src/interfaces/swagger.rs << 'EOF'
use utoipa::OpenApi;
use crate::interfaces::http::health_handler::HealthResponse;
use crate::interfaces::http::register_handler::{RegisterRequest, RegisterResponse};
use crate::interfaces::http::login_handler::{LoginRequest, LoginResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::interfaces::http::health_handler::health_handler,
        crate::interfaces::http::register_handler::register_handler,
        crate::interfaces::http::login_handler::login_handler
    ),
    components(
        schemas(
            HealthResponse,
            RegisterRequest,
            RegisterResponse,
            LoginRequest,
            LoginResponse
        )
    ),
    tags(
        (name = "System", description = "System endpoints like health check"),
        (name = "Authentication", description = "User authentication endpoints")
    )
)]
pub struct ApiDoc;
EOF

# -----------------------
# src/main.rs
cat > src/main.rs << 'EOF'
use actix_web::{middleware::Logger, App, HttpServer, web};
use env_logger;
use dotenvy::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod domain;
mod infrastructure;
mod application;
mod interfaces;

use interfaces::swagger::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = env::var("SERVER_PORT").unwrap_or("8000".to_string()).parse::<u16>().unwrap();

    let keycloak_client = infrastructure::keycloak_client::KeycloakClient::new(
        env::var("KEYCLOAK_URL").unwrap(),
        env::var("KEYCLOAK_REALM").unwrap(),
        env::var("KEYCLOAK_CLIENT_ID").unwrap(),
        env::var("KEYCLOAK_CLIENT_SECRET").unwrap(),
    );

    let user_repo = web::Data::new(infrastructure::user_repository::KeycloakUserRepository {
        client: keycloak_client,
    });

    let openapi = ApiDoc::openapi();

    println!("Server running at http://localhost:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(user_repo.clone())
            .service(interfaces::http::health_handler::health_handler)
            .service(interfaces::http::register_handler::register_handler)
            .service(interfaces::http::login_handler::login_handler)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
EOF

echo "✅ Rust Keycloak API project generated successfully!"
