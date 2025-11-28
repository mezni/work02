#!/bin/bash
# create_infrastructure_layer_fixed.sh
# Creates the Infrastructure layer structure and starter Rust files with corrections

SERVICE_DIR="auth-service"

echo "Applying fixes and enhancements to the Infrastructure layer structure..."

cd $SERVICE_DIR

# --- 1. Create/Fix directories (ensuring all are present)
mkdir -p src/infrastructure/db
mkdir -p src/infrastructure/keycloak
mkdir -p src/infrastructure/jwt
mkdir -p src/infrastructure/ioc # Add IOC/ServiceLocator file

# --- 2. Create mod.rs (Add ioc for Dependency Injection)
cat > src/infrastructure/mod.rs <<EOL
pub mod db;
pub mod keycloak;
pub mod jwt;
pub mod logging;
pub mod config;
pub mod ioc; // Dependency Injection
EOL

# --- 3. Fix logging.rs (Use the function name 'setup_tracing' used in main.rs)
cat > src/infrastructure/logging.rs <<EOL
use tracing_subscriber::{fmt, EnvFilter};

pub fn setup_tracing() {
    // FIX: Use try_from_env to allow dynamic log level based on RUST_LOG env var
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("auth_service=info,actix_web=info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();
}
EOL

# --- 4. Fix config.rs (Use standard Rust/Serde deserialization, match previous structure)
cat > src/infrastructure/config.rs <<EOL
use serde::Deserialize;
use dotenvy::dotenv;

#[derive(Debug, Clone, Deserialize)]
pub struct KeycloakConfig {
    // FIX: Using underscores for compatibility with .env (KEYCLOAK_REALM_URL, etc.)
    pub keycloak_realm_url: String, 
    pub keycloak_client_id: String,
    pub keycloak_client_secret: String,
    pub keycloak_master_user: String,
    pub keycloak_master_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,

    #[serde(flatten)]
    pub keycloak: KeycloakConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, dotenvy::Error> {
        dotenv().ok();
        
        // FIX: Using std::env::var and mapping manually as a simple config loader
        Ok(AppConfig {
            database_url: std::env::var("DATABASE_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            host: std::env::var("HOST")?,
            port: std::env::var("PORT")?.parse().unwrap_or(8000),
            keycloak: KeycloakConfig {
                keycloak_realm_url: std::env::var("KEYCLOAK_REALM_URL")?,
                keycloak_client_id: std::env::var("KEYCLOAK_CLIENT_ID")?,
                keycloak_client_secret: std::env::var("KEYCLOAK_CLIENT_SECRET")?,
                keycloak_master_user: std::env::var("KEYCLOAK_MASTER_USER")?,
                keycloak_master_password: std::env::var("KEYCLOAK_MASTER_PASSWORD")?,
            },
        })
    }
}
EOL

# --- 5. Fix UserRepositoryPg (Implement Repository Trait Correctly)
cat > src/infrastructure/db/user_repository_pg.rs <<EOL
use crate::domain::{
    models::User,
    errors::DomainError,
    repositories::UserRepository,
    value_objects::{Username, Email},
};
use async_trait::async_trait;
use sqlx::{PgPool, Executor, types::uuid::Uuid as SqlxUuid};
use uuid::Uuid;

#[derive(Clone)] // FIX: Repositories must be Cloneable for Actix web::Data
pub struct UserRepositoryPg {
    pool: PgPool,
}

impl UserRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        UserRepositoryPg { pool }
    }
}

// FIX: UserRepository trait now correctly uses Option<User> and takes VOs/Uuids.
#[async_trait]
impl UserRepository for UserRepositoryPg {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        // SQLx implementation stub...
        Ok(None) 
    }

    async fn get_by_keycloak_id(&self, keycloak_id: Uuid) -> Result<Option<User>, DomainError> {
        // SQLx implementation stub...
        Ok(None) 
    }

    async fn get_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        // SQLx implementation stub...
        Ok(None)
    }

    async fn get_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        // SQLx implementation stub...
        Ok(None)
    }

    async fn save(&self, user: User) -> Result<User, DomainError> {
        // SQLx implementation stub (INSERT/UPDATE)...
        Ok(user)
    }
}
EOL
touch src/infrastructure/db/organisation_repository_pg.rs
touch src/infrastructure/db/station_repository_pg.rs

# --- 6. Fix Keycloak Client (Add proper error handling and remove DTO/Domain leaks)
cat > src/infrastructure/keycloak/keycloak_client.rs <<EOL
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use uuid::Uuid;
use crate::{
    domain::value_objects::Role,
    infrastructure::config::KeycloakConfig,
};

// FIX: Define a specific Keycloak error type for the Application Layer to map
#[derive(Debug, thiserror::Error)]
pub enum KeycloakError {
    #[error("Keycloak API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Keycloak API returned error: {0}")]
    ApiError(String),
    #[error("Keycloak authentication failed.")]
    AuthError,
}

// Trait defining the contract for Keycloak interaction (Infrastructure/Gateway)
#[async_trait]
pub trait KeycloakClient: Send + Sync + Clone { // FIX: Must be Cloneable
    async fn login(&self, username: &str, password: &str) -> Result<String, KeycloakError>;
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        role: Role,
    ) -> Result<Uuid, KeycloakError>; // Returns Keycloak ID
    async fn get_user_id_by_username(&self, username: &str) -> Result<Uuid, KeycloakError>;
}

#[derive(Clone)]
pub struct KeycloakClientImpl {
    http_client: HttpClient,
    config: KeycloakConfig,
}

impl KeycloakClientImpl {
    pub fn new(config: KeycloakConfig) -> Self {
        Self { 
            http_client: HttpClient::new(), 
            config,
        }
    }
}

#[async_trait]
impl KeycloakClient for KeycloakClientImpl {
    async fn login(&self, username: &str, password: &str) -> Result<String, KeycloakError> {
        // STUB: Authenticate user against Keycloak and return the raw JWT
        if username == "test" && password == "pass" {
            Ok("keycloak_raw_jwt_stub".to_string())
        } else {
            Err(KeycloakError::AuthError)
        }
    }

    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        role: Role,
    ) -> Result<Uuid, KeycloakError> {
        // STUB: Keycloak API call to create user, assign role/group
        Ok(Uuid::new_v4())
    }
    
    async fn get_user_id_by_username(&self, username: &str) -> Result<Uuid, KeycloakError> {
        // STUB: Keycloak API call to look up user ID
        Ok(Uuid::new_v4())
    }
}
EOL

# --- 7. Fix JWT Enricher (Define claims and remove DTO leak)
cat > src/infrastructure/jwt/token_enricher.rs <<EOL
use jsonwebtoken::{encode, Header, EncodingKey, errors::Error as JwtError};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{
    domain::{
        models::User,
        value_objects::Role,
    },
    infrastructure::config::AppConfig,
};

// FIX: Define a specific JWT Error type
pub use jsonwebtoken::errors::Error as JwtError;

// FIX: Define the Claims structure (Application/Infrastructure boundary)
#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichedClaims {
    pub sub: String, 
    pub exp: usize,  
    pub iat: usize,  
    pub role: String,
    pub organisation_name: Option<String>,
    pub station_name: Option<String>,
    pub preferred_username: String,
}

#[derive(Clone)]
pub struct JwtTokenEnricher {
    encoding_key: EncodingKey,
    // Note: Decoding key would be needed for validation middleware
}

impl JwtTokenEnricher {
    pub fn new(config: &AppConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_ref());
        JwtTokenEnricher { encoding_key }
    }

    // FIX: Takes the *Domain Entity* (User) and returns the *Raw Token String*
    pub async fn enrich_and_encode(&self, user: &User) -> Result<String, JwtError> {
        let expiration = Utc::now() + Duration::hours(1);
        let iat = Utc::now().timestamp() as usize;

        // STUB: Fetch Org/Station names from DB using user IDs (omitted actual DB calls)
        let (org_name, station_name) = match user.role {
            Role::Admin => (None, None),
            Role::Partner => (Some("PartnerOrg_A".to_string()), None),
            Role::Operator => (Some("PartnerOrg_A".to_string()), Some("Station_X".to_string())),
            _ => (None, None),
        };

        let claims = EnrichedClaims {
            sub: user.keycloak_id.to_string(),
            exp: expiration.timestamp() as usize,
            iat,
            role: user.role.to_string(),
            organisation_name: org_name,
            station_name: station_name,
            preferred_username: user.username.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
    }
}
EOL

echo "✅ Infrastructure layer fixed, ensuring correct trait implementation and separation of concerns."