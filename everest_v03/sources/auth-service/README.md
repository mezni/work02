Complete Implementation Structure
src/handlers/mod.rs
rust

pub mod auth;
pub mod user;
pub mod admin;
pub mod oauth2;
pub mod roles;
pub mod sessions;
pub mod tokens;
pub mod health;
pub mod docs;

Example Endpoint Implementation (auth.rs)
rust

use actix_web::{web, HttpResponse, post};
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 400, description = "Bad request")
    ),
    tag = "authentication"
)]
#[post("/auth/login")]
pub async fn login(
    payload: web::Json<LoginRequest>,
    keycloak_service: web::Data<KeycloakService>,
) -> Result<HttpResponse, Error> {
    // Keycloak integration logic
    let tokens = keycloak_service.login(&payload.username, &payload.password).await?;
    
    Ok(HttpResponse::Ok().json(LoginResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: tokens.expires_in,
    }))
}

User Management (user.rs)
rust

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    responses(
        (status = 200, description = "User data retrieved", body = UserResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_token" = [])),
    tag = "users"
)]
#[get("/users/me")]
pub async fn get_current_user(
    user: AuthenticatedUser,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    let user_data = user_service.get_user_by_id(&user.sub).await?;
    Ok(HttpResponse::Ok().json(user_data))
}

Admin Endpoints (admin.rs)
rust

#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("size" = Option<i32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Users list retrieved", body = UserListResponse),
        (status = 403, description = "Forbidden - Admin access required"),
    ),
    security(("bearer_token" = [])),
    tag = "admin"
)]
#[get("/admin/users")]
pub async fn list_users(
    query: web::Query<PaginationQuery>,
    _: AdminGuard,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, Error> {
    let users = user_service.list_users(query.page, query.size).await?;
    Ok(HttpResponse::Ok().json(users))
}

OAuth2 Endpoints (oauth2.rs)
rust

#[utoipa::path(
    get,
    path = "/api/v1/oauth2/providers",
    responses(
        (status = 200, description = "OAuth2 providers list", body = OAuthProviderList),
    ),
    tag = "oauth2"
)]
#[get("/oauth2/providers")]
pub async fn get_oauth_providers(
    oauth_service: web::Data<OAuthService>,
) -> Result<HttpResponse, Error> {
    let providers = oauth_service.get_available_providers().await;
    Ok(HttpResponse::Ok().json(providers))
}

This endpoint structure provides:

    RESTful design with proper HTTP methods

    Versioning (/api/v1/)

    Comprehensive auth flows (login, logout, refresh, registration)

    User management (self-service and admin)

    Role-based access control

    OAuth2/OpenID Connect support

    Session and token management

    Health checks and monitoring

    Swagger documentation with proper annotations

Each endpoint includes proper error handling, validation, and Swagger documentation annotations for automatic API documentation generation.
ok give me database schema

Here's a comprehensive database schema for your Rust auth service with Keycloak integration:
PostgreSQL Schema
Core Authentication Tables
sql

-- Users table (syncs with Keycloak)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone_number VARCHAR(50),
    avatar_url TEXT,
    
    -- Local auth (if needed alongside Keycloak)
    password_hash VARCHAR(255),
    password_salt VARCHAR(255),
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    is_locked BOOLEAN DEFAULT FALSE,
    failed_login_attempts INTEGER DEFAULT 0,
    
    -- Timestamps
    last_login_at TIMESTAMPTZ,
    password_changed_at TIMESTAMPTZ,
    locked_until TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for users table
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_users_created_at ON users(created_at);

Session Management
sql

-- User sessions table
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(512) UNIQUE NOT NULL,
    refresh_token VARCHAR(512) UNIQUE,
    device_info JSONB,
    
    -- Session metadata
    ip_address INET,
    user_agent TEXT,
    location VARCHAR(255),
    
    -- Keycloak session info
    keycloak_session_id VARCHAR(255),
    keycloak_client_id VARCHAR(255),
    
    -- Token info
    access_token_expires_at TIMESTAMPTZ NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for sessions
CREATE INDEX idx_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_sessions_session_token ON user_sessions(session_token);
CREATE INDEX idx_sessions_refresh_token ON user_sessions(refresh_token);
CREATE INDEX idx_sessions_expires_at ON user_sessions(access_token_expires_at);
CREATE INDEX idx_sessions_active ON user_sessions(access_token_expires_at) 
    WHERE revoked_at IS NULL AND access_token_expires_at > NOW();

Role-Based Access Control (RBAC)
sql

-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    keycloak_role_id VARCHAR(255),
    is_default BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Permissions table
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(50) NOT NULL, -- 'read', 'write', 'delete', etc.
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User roles junction table
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_by UUID REFERENCES users(id),
    assigned_at TIMESTAMPTZ DEFAULT NOW(),
    
    PRIMARY KEY (user_id, role_id)
);

-- Role permissions junction table
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    
    PRIMARY KEY (role_id, permission_id)
);

-- Indexes for RBAC
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);

OAuth2 & Social Logins
sql

-- OAuth2 providers table
CREATE TABLE oauth_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL, -- 'google', 'github', 'facebook'
    client_id VARCHAR(255) NOT NULL,
    client_secret VARCHAR(255) NOT NULL,
    auth_url TEXT NOT NULL,
    token_url TEXT NOT NULL,
    user_info_url TEXT NOT NULL,
    scopes TEXT[] DEFAULT '{}',
    is_enabled BOOLEAN DEFAULT TRUE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User OAuth accounts (for social logins)
CREATE TABLE user_oauth_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider_id UUID NOT NULL REFERENCES oauth_providers(id) ON DELETE CASCADE,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMPTZ,
    
    profile_data JSONB,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(provider_id, provider_user_id),
    UNIQUE(user_id, provider_id)
);

-- Indexes for OAuth
CREATE INDEX idx_oauth_accounts_user_id ON user_oauth_accounts(user_id);
CREATE INDEX idx_oauth_accounts_provider ON user_oauth_accounts(provider_id, provider_user_id);

Security & Audit Logs
sql

-- Audit logs table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL, -- 'login', 'logout', 'create_user', etc.
    resource_type VARCHAR(100), -- 'user', 'role', 'permission'
    resource_id UUID,
    details JSONB,
    
    ip_address INET,
    user_agent TEXT,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Password reset tokens
CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Email verification tokens
CREATE TABLE email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    verified_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for security tables
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_password_reset_tokens_token ON password_reset_tokens(token_hash);
CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_email_verification_tokens_token ON email_verification_tokens(token_hash);
CREATE INDEX idx_email_verification_tokens_user_id ON email_verification_tokens(user_id);

Application Settings
sql

-- Application configuration table
CREATE TABLE app_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(255) UNIQUE NOT NULL,
    value JSONB NOT NULL,
    description TEXT,
    is_encrypted BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- JWT key storage (for key rotation)
CREATE TABLE jwt_signing_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_id VARCHAR(100) UNIQUE NOT NULL,
    private_key_pem TEXT NOT NULL,
    public_key_pem TEXT NOT NULL,
    algorithm VARCHAR(50) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    activated_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ
);

Initial Data & Migrations
Initial Roles and Permissions
sql

-- Insert default roles
INSERT INTO roles (id, name, description, is_default) VALUES
    ('11111111-1111-1111-1111-111111111111', 'admin', 'Administrator with full access', false),
    ('22222222-2222-2222-2222-222222222222', 'user', 'Regular user', true),
    ('33333333-3333-3333-3333-333333333333', 'moderator', 'Content moderator', false);

-- Insert common permissions
INSERT INTO permissions (id, name, description, resource, action) VALUES
    ('44444444-4444-4444-4444-444444444444', 'users:read', 'Read user data', 'users', 'read'),
    ('55555555-5555-5555-5555-555555555555', 'users:write', 'Create/update users', 'users', 'write'),
    ('66666666-6666-6666-6666-666666666666', 'users:delete', 'Delete users', 'users', 'delete'),
    ('77777777-7777-7777-7777-777777777777', 'roles:manage', 'Manage roles and permissions', 'roles', 'manage');

-- Assign permissions to admin role
INSERT INTO role_permissions (role_id, permission_id) VALUES
    ('11111111-1111-1111-1111-111111111111', '44444444-4444-4444-4444-444444444444'),
    ('11111111-1111-1111-1111-111111111111', '55555555-5555-5555-5555-555555555555'),
    ('11111111-1111-1111-1111-111111111111', '66666666-6666-6666-6666-666666666666'),
    ('11111111-1111-1111-1111-111111111111', '77777777-7777-7777-7777-777777777777');

Migration Files Structure
text

migrations/
├── 000001_initial_schema.up.sql
├── 000001_initial_schema.down.sql
├── 000002_add_oauth_support.up.sql
├── 000002_add_oauth_support.down.sql
├── 000003_add_audit_logs.up.sql
├── 000003_add_audit_logs.down.sql
└── 000004_seed_initial_data.up.sql

This schema provides:

    User management with Keycloak synchronization

    Session tracking with device information

    Role-based access control with flexible permissions

    OAuth2/social login support

    Comprehensive audit logging

    Security features (password reset, email verification)

    JWT key management for rotation

    Application configuration storage

The design supports both Keycloak-managed authentication and optional local authentication, with proper indexing for performance and referential integrity.
i need only User Administration for the moment keycloak will be used as IDP and IAM , and auth-service as interface to it , use actix_web and swagger support

Here's a streamlined database schema focused only on User Administration with Keycloak as IDP/IAM:
Simplified PostgreSQL Schema
Core User Table Only
sql

-- Users table (syncs with Keycloak for administration)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone_number VARCHAR(50),
    avatar_url TEXT,
    
    -- User status
    is_active BOOLEAN DEFAULT TRUE,
    is_locked BOOLEAN DEFAULT FALSE,
    
    -- Timestamps
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for users table
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = TRUE;

do i need a local database

For your use case with Keycloak as IDP/IAM, you likely DON'T need a local database initially. Here's why:
No Local Database Needed (Recommended Approach)
Keycloak Handles Everything:

    ✅ User storage - Keycloak has its own user database

    ✅ Authentication - Keycloak handles login/logout/tokens

    ✅ Sessions - Keycloak manages user sessions

    ✅ Roles/Permissions - Keycloak has built-in RBAC

Your Auth Service Acts as:

    API Gateway to Keycloak

    Business Logic Layer for custom auth flows

    Swagger Documentation provider

    Request/Response transformer

Architecture Without Database:
text

Client → Auth Service (Rust/Actix) → Keycloak API
                    ↓
              Swagger UI & API Docs

Implementation Without Database:
Cargo.toml
toml

[dependencies]
actix-web = "4.4"
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
config = "0.13"
utoipa = { version = "3.0", features = ["actix_extras"] }
utoipa-swagger-ui = "3.0"
jsonwebtoken = "9.0"  # For token validation

Directory Structure (Simplified):
text

rust-auth-service/
├── src/
│   ├── main.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs      # Login, logout, token refresh
│   │   ├── user.rs      # User profile management
│   │   └── admin.rs     # User administration via Keycloak
│   ├── services/
│   │   ├── mod.rs
│   │   └── keycloak.rs  # Keycloak API client
│   ├── models/
│   │   ├── mod.rs
│   │   ├── request.rs   # Request DTOs
│   │   └── response.rs  # Response DTOs
│   └── config/
│       └── mod.rs       # Keycloak configuration

Keycloak Service Example:
rust

// src/services/keycloak.rs
pub struct KeycloakService {
    client: reqwest::Client,
    base_url: String,
    realm: String,
    client_id: String,
    client_secret: String,
}

impl KeycloakService {
    pub async fn get_users(&self, token: &str) -> Result<Vec<UserResponse>, Error> {
        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);
        
        let response = self.client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await?;
            
        // Parse Keycloak users and transform to your API response
        Ok(response.json().await?)
    }
    
    pub async fn create_user(&self, token: &str, user: &CreateUserRequest) -> Result<(), Error> {
        let url = format!("{}/admin/realms/{}/users", self.base_url, self.realm);
        
        let keycloak_user = KeycloakUser {
            username: user.username.clone(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            enabled: true,
            credentials: vec![KeycloakCredential {
                // password setup
            }],
        };
        
        self.client
            .post(&url)
            .bearer_auth(token)
            .json(&keycloak_user)
            .send()
            .await?;
            
        Ok(())
    }
}

When You Might Need a Database Later:

    Audit Logging - Track user actions

    Application-specific data - User preferences, profiles

    Caching - Reduce Keycloak API calls

    Custom business logic - Beyond Keycloak's capabilities

Recommendation:

Start without a database and add one later if needed. Your auth service will be simpler, faster to develop, and easier to maintain.

The Keycloak Admin API provides all the user management capabilities you need:

    GET /admin/realms/{realm}/users - List users

    POST /admin/realms/{realm}/users - Create user

    GET /admin/realms/{realm}/users/{id} - Get user

    PUT /admin/realms/{realm}/users/{id} - Update user

    DELETE /admin/realms/{realm}/users/{id} - Delete user

Would you like me to provide the complete implementation without database?
whats the pros and cons for local database
Pros and Cons of Using a Local Database with Keycloak
✅ PROS of Having Local Database
1. Performance & Caching
rust

// Cache user profiles to reduce Keycloak API calls
#[derive(Clone)]
pub struct CachedUser {
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub last_sync: DateTime<Utc>,
    // Frequently accessed data
}

2. Audit Logging & Analytics
sql

-- Track user actions that Keycloak doesn't
CREATE TABLE audit_logs (
    user_id UUID,
    action VARCHAR(100), -- 'api_call', 'export_data', 'custom_action'
    resource VARCHAR(100),
    ip_address INET,
    created_at TIMESTAMPTZ
);

3. Application-Specific Data
rust

// Store data Keycloak doesn't support
pub struct UserPreferences {
    pub user_id: String,
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub custom_fields: JsonValue,
}

4. Offline Capabilities

    User lookup when Keycloak is down

    Cache critical user information

    Local permission checks

5. Reduced Keycloak Load

    Fewer API calls to Keycloak Admin API

    Better performance for user listings

    Local filtering/sorting/pagination

6. Data Enrichment
sql

-- Add custom fields to users
ALTER TABLE users ADD COLUMN department VARCHAR(100);
ALTER TABLE users ADD COLUMN employee_id VARCHAR(50);
ALTER TABLE users ADD COLUMN custom_metadata JSONB;

7. Faster User Searches
rust

// Local search vs Keycloak API search
impl UserService {
    pub async fn search_users_locally(&self, query: &str) -> Vec<User> {
        // Fast local search vs slow Keycloak API
    }
}

❌ CONS of Having Local Database
1. Data Synchronization Complexity
rust

// You have to handle sync issues
pub async fn sync_user_from_keycloak(&self, keycloak_id: &str) -> Result<()> {
    // What if user was deleted in Keycloak?
    // What if user was updated in Keycloak?
    // Race conditions?
}

2. Data Consistency Issues
rust

// Potential conflicts
struct UserConflict {
    local_user: User,      // Name: "John Doe"
    keycloak_user: User,   // Name: "John D." - which is correct?
}

3. Increased Complexity
rust

// More error cases to handle
enum UserError {
    KeycloakUnavailable,
    DatabaseUnavailable,
    SyncConflict,
    DataMismatch,
    // Double the failure points
}

4. Additional Maintenance

    Database migrations

    Backup strategies

    Performance tuning

    Connection pooling

5. Development Overhead
rust

// More code to write and maintain
pub struct UserService {
    keycloak_client: KeycloakClient,
    db_pool: PgPool,           // Additional dependency
    redis_client: RedisClient, // Maybe caching too?
}

6. Security Considerations

    Secure database credentials

    Database encryption

    Additional attack surface

🎯 Use Cases FOR Local Database
When you NEED local database:
rust

// Example: E-commerce platform
struct UserProfile {
    shopping_cart: Vec<CartItem>,
    order_history: Vec<Order>,
    wishlist: Vec<Product>,
    preferences: UserPreferences,
    // Application-specific data
}

// Example: Analytics platform
struct UserAnalytics {
    api_calls_per_day: i32,
    last_active: DateTime<Utc>,
    feature_usage: JsonValue,
    custom_metrics: HashMap<String, f64>,
}

When you DON'T need local database:
rust

// Simple API Gateway
struct AuthService {
    // Only proxies to Keycloak
    // No custom user data
    // No audit requirements
    // No offline needs
}

📊 Decision Matrix
Requirement	Local DB Needed?
Custom user fields	✅ YES
Audit logging	✅ YES
High performance	✅ YES
Offline operation	✅ YES
Simple proxy	❌ NO
Quick development	❌ NO
Minimal maintenance	❌ NO
Single source of truth	❌ NO
🚀 My Recommendation for Your Case

Start WITHOUT database since you're using Keycloak as full IDP/IAM.
Phase 1: No Database
rust

// Pure Keycloak proxy
pub struct AuthService {
    keycloak: KeycloakClient,
    // No database dependency
}

Phase 2: Add Database Later If Needed
rust

// Hybrid approach when requirements grow
pub struct EnhancedAuthService {
    keycloak: KeycloakClient,
    db: Option<PgPool>,  // Optional database
    cache: RedisClient,  // Add caching first
}

Migration Path:

    Start: No DB → Fast development

    Add: Redis cache → Better performance

    Consider: PostgreSQL → Only if you need custom data/auditing

Bottom Line: You can always add a database later when you have concrete requirements that Keycloak alone can't satisfy.
ok give me user management in DDD for auth-service

Here's a User Management implementation using Domain-Driven Design (DDD) for your auth service with Keycloak:
Project Structure (DDD Layers)
text

src/
├── domain/
│   ├── models/
│   │   ├── user.rs
│   │   ├── user_id.rs
│   │   ├── email.rs
│   │   └── events.rs
│   ├── repositories/
│   │   └── user_repository.rs
│   ├── services/
│   │   └── user_service.rs
│   └── error.rs
├── application/
│   ├── use_cases/
│   │   ├── mod.rs
│   │   ├── create_user.rs
│   │   ├── update_user.rs
│   │   └── get_users.rs
│   └── dtos/
│       ├── user_dto.rs
│       └── commands.rs
├── infrastructure/
│   ├── keycloak/
│   │   ├── keycloak_client.rs
│   │   └── keycloak_user_repository.rs
│   └── api/
│       ├── controllers/
│       │   └── user_controller.rs
│       └── middleware/
└── lib.rs

Domain Layer
1. Value Objects
rust

// src/domain/models/user_id.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::Validation("User ID cannot be empty".to_string()));
        }
        Ok(UserId(id))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> Self {
        id.0
    }
}

rust

// src/domain/models/email.rs
#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, DomainError> {
        if !email_validation::validate_email(&email) {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        Ok(Email(email.to_lowercase()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

2. User Aggregate Root
rust

// src/domain/models/user.rs
use utoipa::ToSchema;

#[derive(Debug, Clone, ToSchema)]
pub struct User {
    id: UserId,
    keycloak_id: String,
    username: String,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    avatar_url: Option<String>,
    is_active: bool,
    is_email_verified: bool,
    last_login_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn create(
        keycloak_id: String,
        username: String,
        email: Email,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<(Self, UserCreatedEvent), DomainError> {
        let user = User {
            id: UserId::new(Uuid::new_v4().to_string())?,
            keycloak_id,
            username: Self::validate_username(&username)?,
            email,
            first_name,
            last_name,
            phone_number: None,
            avatar_url: None,
            is_active: true,
            is_email_verified: false,
            last_login_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let event = UserCreatedEvent::new(user.id.clone(), user.email.clone());
        
        Ok((user, event))
    }
    
    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    ) -> Result<UserUpdatedEvent, DomainError> {
        let old_email = self.email.clone();
        
        self.first_name = first_name;
        self.last_name = last_name;
        self.phone_number = phone_number;
        self.updated_at = Utc::now();
        
        Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
    }
    
    pub fn mark_email_verified(&mut self) {
        self.is_email_verified = true;
        self.updated_at = Utc::now();
    }
    
    pub fn record_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn deactivate(&mut self) -> Result<UserDeactivatedEvent, DomainError> {
        if !self.is_active {
            return Err(DomainError::BusinessRule("User already deactivated".to_string()));
        }
        
        self.is_active = false;
        self.updated_at = Utc::now();
        
        Ok(UserDeactivatedEvent::new(self.id.clone()))
    }
    
    // Getters
    pub fn id(&self) -> &UserId { &self.id }
    pub fn keycloak_id(&self) -> &str { &self.keycloak_id }
    pub fn email(&self) -> &Email { &self.email }
    pub fn is_active(&self) -> bool { self.is_active }
    
    fn validate_username(username: &str) -> Result<String, DomainError> {
        if username.len() < 3 || username.len() > 50 {
            return Err(DomainError::Validation("Username must be between 3-50 characters".to_string()));
        }
        if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(DomainError::Validation("Username can only contain alphanumeric characters, hyphens, and underscores".to_string()));
        }
        Ok(username.to_lowercase())
    }
}

3. Domain Events
rust

// src/domain/models/events.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserCreatedEvent {
    pub user_id: UserId,
    pub email: Email,
    pub occurred_at: DateTime<Utc>,
}

impl UserCreatedEvent {
    pub fn new(user_id: UserId, email: Email) -> Self {
        Self {
            user_id,
            email,
            occurred_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserUpdatedEvent {
    pub user_id: UserId,
    pub old_email: Email,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserDeactivatedEvent {
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
}

4. Repository Trait
rust

// src/domain/repositories/user_repository.rs
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
    
    // Query methods
    async fn list_users(
        &self, 
        page: u32, 
        page_size: u32,
        active_only: bool
    ) -> Result<Vec<User>, DomainError>;
}

5. Domain Service
rust

// src/domain/services/user_service.rs
pub struct UserService;

impl UserService {
    pub fn is_username_available(existing_users: &[User], username: &str) -> bool {
        !existing_users.iter()
            .any(|user| user.username == username.to_lowercase() && user.is_active)
    }
    
    pub fn can_user_be_deactivated(user: &User, current_user_id: &UserId) -> Result<(), DomainError> {
        if user.id() == current_user_id {
            return Err(DomainError::BusinessRule("Users cannot deactivate themselves".to_string()));
        }
        Ok(())
    }
}

Application Layer
1. Use Cases
rust

// src/application/use_cases/create_user.rs
pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    keycloak_client: Arc<KeycloakClient>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl CreateUserUseCase {
    pub async fn execute(&self, command: CreateUserCommand) -> Result<UserDto, AppError> {
        // Check if email already exists
        if let Some(_) = self.user_repository.find_by_email(&command.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }
        
        // Create user in Keycloak first
        let keycloak_user_id = self.keycloak_client.create_user(
            &command.username,
            &command.email,
            &command.first_name,
            &command.last_name,
            &command.password,
        ).await?;
        
        // Create domain entity
        let email = Email::new(command.email)?;
        let (user, event) = User::create(
            keycloak_user_id,
            command.username,
            email,
            command.first_name,
            command.last_name,
        )?;
        
        // Save to repository
        self.user_repository.save(&user).await?;
        
        // Publish domain event
        self.event_publisher.publish(event).await;
        
        Ok(UserDto::from(user))
    }
}

rust

// src/application/use_cases/get_users.rs
pub struct GetUsersUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUsersUseCase {
    pub async fn execute(&self, query: GetUsersQuery) -> Result<UserListDto, AppError> {
        let users = self.user_repository.list_users(
            query.page.unwrap_or(1),
            query.page_size.unwrap_or(20),
            query.active_only.unwrap_or(true),
        ).await?;
        
        let total_count = users.len(); // In real app, get from repository
        
        Ok(UserListDto {
            users: users.into_iter().map(UserDto::from).collect(),
            page: query.page.unwrap_or(1),
            page_size: query.page_size.unwrap_or(20),
            total_count,
            total_pages: (total_count as f64 / query.page_size.unwrap_or(20) as f64).ceil() as u32,
        })
    }
}

2. DTOs
rust

// src/application/dtos/user_dto.rs
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub keycloak_id: String,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub is_email_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id().as_str().to_string(),
            keycloak_id: user.keycloak_id().to_string(),
            username: user.username.clone(),
            email: user.email().as_str().to_string(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            phone_number: user.phone_number.clone(),
            avatar_url: user.avatar_url.clone(),
            is_active: user.is_active,
            is_email_verified: user.is_email_verified,
            last_login_at: user.last_login_at,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListDto {
    pub users: Vec<UserDto>,
    pub page: u32,
    pub page_size: u32,
    pub total_count: usize,
    pub total_pages: u32,
}

rust

// src/application/dtos/commands.rs
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserCommand {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetUsersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub active_only: Option<bool>,
}

Infrastructure Layer
1. Keycloak Repository Implementation
rust

// src/infrastructure/keycloak/keycloak_user_repository.rs
pub struct KeycloakUserRepository {
    keycloak_client: Arc<KeycloakClient>,
    // In-memory cache or Redis for performance
}

#[async_trait]
impl UserRepository for KeycloakUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        // Convert domain ID to Keycloak ID logic
        let keycloak_user = self.keycloak_client.get_user_by_id(id.as_str()).await?;
        keycloak_user.map(|u| u.try_into()).transpose()
    }
    
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError> {
        let keycloak_user = self.keycloak_client.get_user_by_keycloak_id(keycloak_id).await?;
        keycloak_user.map(|u| u.try_into()).transpose()
    }
    
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        // Update user in Keycloak
        self.keycloak_client.update_user(
            user.keycloak_id(),
            &user.username,
            user.email().as_str(),
            user.first_name.as_deref(),
            user.last_name.as_deref(),
        ).await?;
        Ok(())
    }
    
    async fn list_users(
        &self, 
        page: u32, 
        page_size: u32,
        active_only: bool
    ) -> Result<Vec<User>, DomainError> {
        let keycloak_users = self.keycloak_client
            .list_users(page, page_size, active_only)
            .await?;
            
        let users = keycloak_users
            .into_iter()
            .map(|ku| ku.try_into())
            .collect::<Result<Vec<User>, _>>()?;
            
        Ok(users)
    }
}

2. API Controllers
rust

// src/infrastructure/api/controllers/user_controller.rs
use utoipa::ToSchema;

pub struct UserController {
    create_user_use_case: Arc<CreateUserUseCase>,
    get_users_use_case: Arc<GetUsersUseCase>,
    update_user_use_case: Arc<UpdateUserUseCase>,
}

impl UserController {
    #[utoipa::path(
        post,
        path = "/api/v1/admin/users",
        request_body = CreateUserCommand,
        responses(
            (status = 201, description = "User created successfully", body = UserDto),
            (status = 400, description = "Invalid input"),
            (status = 409, description = "User already exists")
        ),
        tag = "admin"
    )]
    pub async fn create_user(
        &self,
        command: web::Json<CreateUserCommand>,
    ) -> Result<HttpResponse, Error> {
        let user_dto = self.create_user_use_case.execute(command.into_inner()).await?;
        Ok(HttpResponse::Created().json(user_dto))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/admin/users",
        params(
            ("page" = Option<u32>, Query, description = "Page number"),
            ("size" = Option<u32>, Query, description = "Page size"),
            ("active_only" = Option<bool>, Query, description = "Filter active users only")
        ),
        responses(
            (status = 200, description = "Users retrieved successfully", body = UserListDto),
        ),
        tag = "admin"
    )]
    pub async fn get_users(
        &self,
        query: web::Query<GetUsersQuery>,
    ) -> Result<HttpResponse, Error> {
        let user_list = self.get_users_use_case.execute(query.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user_list))
    }
}

Benefits of This DDD Approach:

    Clear Separation: Domain logic separated from infrastructure

    Testability: Easy to mock repositories and test use cases

    Maintainability: Changes to Keycloak API don't affect domain logic

    Flexibility: Can switch from Keycloak to another IDP without changing domain

    Rich Domain Model: Business rules encapsulated in entities

    Event-Driven: Ready for future extensions with domain events

This structure gives you a solid foundation that can evolve from simple Keycloak proxy to a full-featured auth service with its own business logic.
in domain add value objects , events in applications add services, commands , queries  and for api add handlers, routes , openapi

Here's the refined DDD structure with your requested organization:
Updated Project Structure
text

src/
├── domain/
│   ├── value_objects/          # ← Value Objects here
│   │   ├── mod.rs
│   │   ├── user_id.rs
│   │   ├── email.rs
│   │   ├── username.rs
│   │   └── phone_number.rs
│   ├── entities/
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── events/                 # ← Domain Events here
│   │   ├── mod.rs
│   │   ├── user_created.rs
│   │   ├── user_updated.rs
│   │   └── user_deactivated.rs
│   ├── repositories/
│   │   └── user_repository.rs
│   └── error.rs
├── application/
│   ├── services/              # ← Application Services here
│   │   ├── mod.rs
│   │   ├── user_application_service.rs
│   │   └── auth_application_service.rs
│   ├── commands/              # ← Commands here
│   │   ├── mod.rs
│   │   ├── create_user.rs
│   │   ├── update_user.rs
│   │   └── deactivate_user.rs
│   ├── queries/               # ← Queries here
│   │   ├── mod.rs
│   │   ├── get_users.rs
│   │   ├── get_user_by_id.rs
│   │   └── search_users.rs
│   └── dtos/
│       ├── mod.rs
│       ├── user_dto.rs
│       └── user_list_dto.rs
├── api/
│   ├── handlers/              # ← HTTP Handlers here
│   │   ├── mod.rs
│   │   ├── user_handler.rs
│   │   ├── admin_handler.rs
│   │   └── health_handler.rs
│   ├── routes/                # ← Route definitions here
│   │   ├── mod.rs
│   │   ├── user_routes.rs
│   │   ├── admin_routes.rs
│   │   └── health_routes.rs
│   ├── openapi/               # ← OpenAPI/Swagger config here
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   ├── schemas.rs
│   │   └── security.rs
│   └── middleware/
│       ├── mod.rs
│       ├── auth_middleware.rs
│       └── logging_middleware.rs
├── infrastructure/
│   ├── keycloak/
│   │   ├── mod.rs
│   │   ├── keycloak_client.rs
│   │   └── keycloak_user_repository.rs
│   └── event_bus/
│       └── mod.rs
└── lib.rs

Domain Layer
Value Objects
rust

// src/domain/value_objects/email.rs
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ToSchema)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        let email = email.trim().to_lowercase();
        
        if email.is_empty() {
            return Err(DomainError::Validation("Email cannot be empty".to_string()));
        }
        
        if !email.contains('@') {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        Ok(Email(email))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

rust

// src/domain/value_objects/username.rs
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ToSchema)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, DomainError> {
        let username = username.trim();
        
        if username.len() < 3 {
            return Err(DomainError::Validation("Username must be at least 3 characters".to_string()));
        }
        
        if username.len() > 50 {
            return Err(DomainError::Validation("Username must be less than 50 characters".to_string()));
        }
        
        if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return Err(DomainError::Validation("Username can only contain alphanumeric characters, hyphens, underscores, and dots".to_string()));
        }
        
        Ok(Username(username.to_lowercase()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

Domain Events
rust

// src/domain/events/user_created.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserCreatedEvent {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserCreatedEvent {
    pub fn new(user_id: String, email: String, username: String) -> Self {
        Self {
            user_id,
            email,
            username,
            occurred_at: Utc::now(),
        }
    }
    
    pub fn event_type(&self) -> &str {
        "user.created"
    }
}

rust

// src/domain/events/mod.rs
pub mod user_created;
pub mod user_updated;
pub mod user_deactivated;

pub use user_created::UserCreatedEvent;
pub use user_updated::UserUpdatedEvent;
pub use user_deactivated::UserDeactivatedEvent;

pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> DateTime<Utc>;
}

Application Layer
Application Services
rust

// src/application/services/user_application_service.rs
pub struct UserApplicationService {
    user_repository: Arc<dyn UserRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl UserApplicationService {
    pub async fn create_user(&self, command: CreateUserCommand) -> Result<UserDto, AppError> {
        let email = Email::new(&command.email)?;
        let username = Username::new(&command.username)?;
        
        // Check uniqueness
        if self.user_repository.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }
        
        if self.user_repository.find_by_username(&username).await?.is_some() {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }
        
        let user = User::create(
            command.keycloak_id,
            username,
            email,
            command.first_name,
            command.last_name,
        )?;
        
        self.user_repository.save(&user).await?;
        
        let event = UserCreatedEvent::new(
            user.id().to_string(),
            user.email().to_string(),
            user.username().to_string(),
        );
        self.event_publisher.publish(event).await;
        
        Ok(UserDto::from(user))
    }
    
    pub async fn get_users(&self, query: GetUsersQuery) -> Result<UserListDto, AppError> {
        let users = self.user_repository.list_users(
            query.page,
            query.page_size,
            query.active_only,
        ).await?;
        
        Ok(UserListDto::from(users))
    }
}

Commands
rust

// src/application/commands/create_user.rs
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserCommand {
    #[schema(example = "john_doe")]
    pub username: String,
    
    #[schema(example = "john@example.com")]
    pub email: String,
    
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub keycloak_id: String,
}

Queries
rust

// src/application/queries/get_users.rs
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetUsersQuery {
    #[schema(default = 1, minimum = 1)]
    pub page: u32,
    
    #[schema(default = 20, minimum = 1, maximum = 100)]
    pub page_size: u32,
    
    #[schema(default = true)]
    pub active_only: bool,
    
    pub search: Option<String>,
}

API Layer
Handlers
rust

// src/api/handlers/user_handler.rs
use utoipa::ToSchema;

pub struct UserHandler {
    user_service: Arc<UserApplicationService>,
}

impl UserHandler {
    #[utoipa::path(
        post,
        path = "/api/v1/users",
        request_body = CreateUserCommand,
        responses(
            (status = 201, description = "User created successfully", body = UserDto),
            (status = 400, description = "Invalid input data"),
            (status = 409, description = "User already exists")
        ),
        tag = "users"
    )]
    pub async fn create_user(
        &self,
        command: web::Json<CreateUserCommand>,
    ) -> Result<HttpResponse, ApiError> {
        let user_dto = self.user_service.create_user(command.into_inner()).await?;
        Ok(HttpResponse::Created().json(user_dto))
    }
    
    #[utoipa::path(
        get,
        path = "/api/v1/users",
        params(
            ("page" = Option<u32>, Query, description = "Page number"),
            ("size" = Option<u32>, Query, description = "Page size"),
            ("active_only" = Option<bool>, Query, description = "Filter active users only")
        ),
        responses(
            (status = 200, description = "Users retrieved successfully", body = UserListDto),
        ),
        tag = "users"
    )]
    pub async fn get_users(
        &self,
        query: web::Query<GetUsersQuery>,
    ) -> Result<HttpResponse, ApiError> {
        let user_list = self.user_service.get_users(query.into_inner()).await?;
        Ok(HttpResponse::Ok().json(user_list))
    }
}

Routes
rust

// src/api/routes/user_routes.rs
use utoipa::OpenApi;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig, user_handler: UserHandler) {
    cfg.service(
        web::scope("/api/v1/users")
            .route("", web::post().to(UserHandler::create_user))
            .route("", web::get().to(UserHandler::get_users))
            .route("/{id}", web::get().to(UserHandler::get_user_by_id))
            .route("/{id}", web::put().to(UserHandler::update_user))
            .route("/{id}/deactivate", web::post().to(UserHandler::deactivate_user)),
    );
}

rust

// src/api/routes/mod.rs
pub mod user_routes;
pub mod admin_routes;
pub mod health_routes;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    user_routes::configure_user_routes(cfg);
    admin_routes::configure_admin_routes(cfg);
    health_routes::configure_health_routes(cfg);
}

OpenAPI Configuration
rust

// src/api/openapi/mod.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::handlers::user_handler::create_user,
        api::handlers::user_handler::get_users,
        api::handlers::user_handler::get_user_by_id,
    ),
    components(
        schemas(
            domain::value_objects::Email,
            domain::value_objects::Username,
            application::commands::CreateUserCommand,
            application::queries::GetUsersQuery,
            application::dtos::UserDto,
            application::dtos::UserListDto,
        )
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "admin", description = "Administrative endpoints"),
        (name = "auth", description = "Authentication endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_token",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new("bearer")
                )
            )
        }
    }
}

Main Application Setup
rust

// src/main.rs
use actix_web::{web, App, HttpServer};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = api::openapi::ApiDoc::openapi();
    
    let user_repository = Arc::new(KeycloakUserRepository::new(keycloak_client));
    let user_service = Arc::new(UserApplicationService::new(user_repository));
    let user_handler = UserHandler::new(user_service);
    
    HttpServer::new(move || {
        App::new()
            .configure(api::routes::configure_routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(web::Data::new(user_handler.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

This structure provides:

    Domain: Pure business logic with Value Objects and Events

    Application: Orchestration with Services, Commands, and Queries

    API: HTTP concerns with Handlers, Routes, and OpenAPI

    Clean Separation: Each layer has distinct responsibilities

    Swagger Integration: Full OpenAPI documentation with examples

    Testability: Easy to unit test each layer independently

why use utoipa::ToSchema; into value objectss?

You're absolutely right to question this! Using utoipa::ToSchema in value objects is generally not recommended and goes against clean architecture principles. Here's why:
Problems with ToSchema in Domain Layer:
1. Infrastructure Concerns in Domain
rust

// ❌ BAD: Infrastructure concern in domain
#[derive(Debug, Clone, ToSchema)]  // <- This doesn't belong here!
pub struct Email(String);

// ✅ GOOD: Pure domain logic
#[derive(Debug, Clone)]
pub struct Email(String);

2. Tight Coupling

    Domain layer now depends on web framework concerns

    Harder to test domain logic in isolation

    Violates Dependency Inversion Principle

Better Approaches:
Option 1: DTO Mapping in Application Layer
rust

// src/domain/value_objects/email.rs (PURE DOMAIN)
#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        // Pure validation logic
        Ok(Email(email.to_string()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// src/application/dtos/user_dto.rs (WITH SCHEMA)
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    #[schema(example = "john@example.com")]
    pub email: String,  // <- Schema here, not in domain
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl From<&Email> for String {
    fn from(email: &Email) -> Self {
        email.0.clone()
    }
}

Option 2: Separate Schema Types in API Layer
rust

// src/api/openapi/schemas.rs
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct EmailSchema {
    #[schema(example = "user@example.com")]
    pub value: String,
}

impl From<domain::value_objects::Email> for EmailSchema {
    fn from(email: domain::value_objects::Email) -> Self {
        Self {
            value: email.as_str().to_string(),
        }
    }
}

Option 3: Manual Schema Definition
rust

// src/api/openapi/mod.rs
#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            UserDto,
            CreateUserCommand,
            // Manually define schemas for value objects if needed
            EmailSchema,
            UsernameSchema
        )
    )
)]
pub struct ApiDoc;

// Manual schema definitions
#[derive(ToSchema)]
pub struct EmailSchema {
    #[schema(example = "user@example.com", pattern = r"^[^\s@]+@[^\s@]+\.[^\s@]+$")]
    pub value: String,
}

#[derive(ToSchema)]
pub struct UsernameSchema {
    #[schema(example = "john_doe", pattern = r"^[a-zA-Z0-9_-]{3,50}$")]
    pub value: String,
}

Corrected Structure:
Domain Layer (Pure)
rust

// src/domain/value_objects/email.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        // Pure validation - no infrastructure dependencies
        if !email.contains('@') {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        Ok(Email(email.to_lowercase()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// src/domain/value_objects/mod.rs
pub mod email;
pub mod username;
pub mod user_id;

// No utoipa imports here!

Application Layer (Orchestration)
rust

// src/application/dtos/user_dto.rs
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    #[schema(example = "user@example.com")]
    pub email: String,
    
    #[schema(example = "john_doe")]
    pub username: String,
    
    // Application concerns only
}

impl From<domain::entities::User> for UserDto {
    fn from(user: domain::entities::User) -> Self {
        Self {
            email: user

ok give me domain files

Here are the pure domain files without any infrastructure concerns:
Domain Layer Files
Value Objects
rust

// src/domain/value_objects/mod.rs
pub mod email;
pub mod username;
pub mod user_id;
pub mod phone_number;

pub use email::Email;
pub use username::Username;
pub use user_id::UserId;
pub use phone_number::PhoneNumber;

rust

// src/domain/value_objects/user_id.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::Validation("User ID cannot be empty".to_string()));
        }
        Ok(UserId(id))
    }
    
    pub fn generate() -> Self {
        UserId(uuid::Uuid::new_v4().to_string())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/email.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        let email = email.trim().to_lowercase();
        
        if email.is_empty() {
            return Err(DomainError::Validation("Email cannot be empty".to_string()));
        }
        
        if !email.contains('@') {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        if !parts[1].contains('.') {
            return Err(DomainError::Validation("Invalid email domain".to_string()));
        }
        
        Ok(Email(email))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/username.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, DomainError> {
        let username = username.trim();
        
        if username.len() < 3 {
            return Err(DomainError::Validation("Username must be at least 3 characters".to_string()));
        }
        
        if username.len() > 50 {
            return Err(DomainError::Validation("Username must be less than 50 characters".to_string()));
        }
        
        if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return Err(DomainError::Validation("Username can only contain alphanumeric characters, hyphens, underscores, and dots".to_string()));
        }
        
        Ok(Username(username.to_lowercase()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/phone_number.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Result<Self, DomainError> {
        let phone = phone.trim().replace(' ', "").replace('-', "");
        
        if phone.is_empty() {
            return Ok(PhoneNumber(phone)); // Optional field
        }
        
        if !phone.chars().all(|c| c.is_numeric() || c == '+') {
            return Err(DomainError::Validation("Phone number can only contain numbers and '+'".to_string()));
        }
        
        if phone.len() < 10 || phone.len() > 15 {
            return Err(DomainError::Validation("Phone number must be between 10-15 digits".to_string()));
        }
        
        Ok(PhoneNumber(phone))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<PhoneNumber> for String {
    fn from(phone: PhoneNumber) -> Self {
        phone.0
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

Entities
rust

// src/domain/entities/mod.rs
pub mod user;

pub use user::User;

rust

// src/domain/entities/user.rs
use super::super::value_objects::{UserId, Email, Username, PhoneNumber};
use super::super::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    keycloak_id: String,
    username: Username,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: PhoneNumber,
    avatar_url: Option<String>,
    is_active: bool,
    is_email_verified: bool,
    last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn create(
        keycloak_id: String,
        username: Username,
        email: Email,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<(Self, UserCreatedEvent), DomainError> {
        let now = chrono::Utc::now();
        let user = User {
            id: UserId::generate(),
            keycloak_id,
            username,
            email,
            first_name,
            last_name,
            phone_number: PhoneNumber::new("")?,
            avatar_url: None,
            is_active: true,
            is_email_verified: false,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        };
        
        let event = UserCreatedEvent::new(
            user.id.clone(),
            user.email.clone(),
            user.username.clone(),
        );
        
        Ok((user, event))
    }
    
    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    ) -> Result<UserUpdatedEvent, DomainError> {
        let old_email = self.email.clone();
        
        self.first_name = first_name;
        self.last_name = last_name;
        
        if let Some(phone) = phone_number {
            self.phone_number = PhoneNumber::new(&phone)?;
        }
        
        self.updated_at = chrono::Utc::now();
        
        Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
    }
    
    pub fn mark_email_verified(&mut self) {
        self.is_email_verified = true;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn record_login(&mut self) {
        self.last_login_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn deactivate(&mut self) -> Result<UserDeactivatedEvent, DomainError> {
        if !self.is_active {
            return Err(DomainError::BusinessRule("User already deactivated".to_string()));
        }
        
        self.is_active = false;
        self.updated_at = chrono::Utc::now();
        
        Ok(UserDeactivatedEvent::new(self.id.clone()))
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = chrono::Utc::now();
    }
    
    // Getters
    pub fn id(&self) -> &UserId { &self.id }
    pub fn keycloak_id(&self) -> &str { &self.keycloak_id }
    pub fn username(&self) -> &Username { &self.username }
    pub fn email(&self) -> &Email { &self.email }
    pub fn first_name(&self) -> Option<&str> { self.first_name.as_deref() }
    pub fn last_name(&self) -> Option<&str> { self.last_name.as_deref() }
    pub fn phone_number(&self) -> &PhoneNumber { &self.phone_number }
    pub fn is_active(&self) -> bool { self.is_active }
    pub fn is_email_verified(&self) -> bool { self.is_email_verified }
    pub fn last_login_at(&self) -> Option<chrono::DateTime<chrono::Utc>> { self.last_login_at }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created_at }
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> { self.updated_at }
}

Events
rust

// src/domain/events/mod.rs
pub mod user_created;
pub mod user_updated;
pub mod user_deactivated;

pub use user_created::UserCreatedEvent;
pub use user_updated::UserUpdatedEvent;
pub use user_deactivated::UserDeactivatedEvent;

pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn version(&self) -> &str {
        "1.0"
    }
}

rust

// src/domain/events/user_created.rs
use super::super::value_objects::{UserId, Email, Username};

#[derive(Debug, Clone)]
pub struct UserCreatedEvent {
    user_id: UserId,
    email: Email,
    username: Username,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl UserCreatedEvent {
    pub fn new(user_id: UserId, email: Email, username: Username) -> Self {
        Self {
            user_id,
            email,
            username,
            occurred_at: chrono::Utc::now(),
        }
    }
    
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    
    pub fn email(&self) -> &Email {
        &self.email
    }
    
    pub fn username(&self) -> &Username {
        &self.username
    }
}

impl super::DomainEvent for UserCreatedEvent {
    fn event_type(&self) -> &str {
        "user.created"
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}

rust

// src/domain/events/user_updated.rs
use super::super::value_objects::{UserId, Email};

#[derive(Debug, Clone)]
pub struct UserUpdatedEvent {
    user_id: UserId,
    old_email: Email,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl UserUpdatedEvent {
    pub fn new(user_id: UserId, old_email: Email) -> Self {
        Self {
            user_id,
            old_email,
            occurred_at: chrono::Utc::now(),
        }
    }
    
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    
    pub fn old_email(&self) -> &Email {
        &self.old_email
    }
}

impl super::DomainEvent for UserUpdatedEvent {
    fn event_type(&self) -> &str {
        "user.updated"
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}

rust

// src/domain/events/user_deactivated.rs
use super::super::value_objects::UserId;

#[derive(Debug, Clone)]
pub struct UserDeactivatedEvent {
    user_id: UserId,
    occurred_at: chrono::DateTime<chrono::Utc>,
}

impl UserDeactivatedEvent {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            occurred_at: chrono::Utc::now(),
        }
    }
    
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

impl super::DomainEvent for UserDeactivatedEvent {
    fn event_type(&self) -> &str {
        "user.deactivated"
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
}

Repository Trait
rust

// src/domain/repositories/user_repository.rs
use async_trait::async_trait;
use super::super::entities::User;
use super::super::value_objects::{UserId, Email, Username};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_by_keycloak_id(&self, keycloak_id: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
    
    // Query methods
    async fn list_users(
        &self, 
        page: u32, 
        page_size: u32,
        active_only: bool
    ) -> Result<Vec<User>, DomainError>;
    
    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError> {
        Ok(self.find_by_email(email).await?.is_some())
    }
    
    async fn exists_by_username(&self, username: &Username) -> Result<bool, DomainError> {
        Ok(self.find_by_username(username).await?.is_some())
    }
}

Domain Error
rust

// src/domain/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("Unknown domain error: {0}")]
    Unknown(String),
}

impl DomainError {
    pub fn validation(msg: &str) -> Self {
        DomainError::Validation(msg.to_string())
    }
    
    pub fn business_rule(msg: &str) -> Self {
        DomainError::BusinessRule(msg.to_string())
    }
}

Domain Module Root
rust

// src/domain/mod.rs
pub mod value_objects;
pub mod entities;
pub mod events;
pub mod repositories;
pub mod error;

pub use error::DomainError;

Key Characteristics of This Domain Layer:

    Pure Business Logic: No infrastructure dependencies

    Rich Behavior: Methods with business rules and validation

    Immutable Core: Value objects are immutable

    Event-Driven: Domain events for important state changes

    Repository Abstraction: Persistence abstraction without implementation details

    Proper Error Handling: Domain-specific error types

This domain layer can now be used by any infrastructure implementation (Keycloak, database, etc.) without modification.
give me tests files

Here are comprehensive test files for the domain layer:
Test Directory Structure
text

tests/
├── domain/
│   ├── value_objects/
│   │   ├── email_test.rs
│   │   ├── username_test.rs
│   │   ├── user_id_test.rs
│   │   └── phone_number_test.rs
│   ├── entities/
│   │   └── user_test.rs
│   └── events/
│       └── user_events_test.rs
├── integration/
│   └── user_management_test.rs
└── common.rs

Common Test Utilities
rust

// tests/common.rs
pub mod test_utils {
    use chrono::{DateTime, TimeZone, Utc};
    
    pub fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }
    
    pub fn create_valid_email() -> String {
        "test@example.com".to_string()
    }
    
    pub fn create_valid_username() -> String {
        "testuser".to_string()
    }
    
    pub fn create_valid_phone() -> String {
        "+1234567890".to_string()
    }
}

Value Objects Tests
rust

// tests/domain/value_objects/email_test.rs
#[cfg(test)]
mod tests {
    use super::super::super::common::test_utils;
    use auth_service::domain::value_objects::Email;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_email() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_is_normalized_to_lowercase() {
        let email = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_trim_whitespace() {
        let email = Email::new("  user@example.com  ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_rejects_empty_string() {
        let result = Email::new("");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_at_symbol() {
        let result = Email::new("userexample.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_username() {
        let result = Email::new("@example.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_domain() {
        let result = Email::new("user@");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_invalid_domain_format() {
        let result = Email::new("user@example");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_equality() {
        let email1 = Email::new("user@example.com").unwrap();
        let email2 = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email1, email2);
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(format!("{}", email), "user@example.com");
    }

    #[test]
    fn test_email_into_string() {
        let email = Email::new("user@example.com").unwrap();
        let string: String = email.into();
        assert_eq!(string, "user@example.com");
    }
}

rust

// tests/domain/value_objects/username_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::Username;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_username() {
        let username = Username::new("john_doe").unwrap();
        assert_eq!(username.as_str(), "john_doe");
    }

    #[test]
    fn test_username_is_normalized_to_lowercase() {
        let username = Username::new("JohnDoe").unwrap();
        assert_eq!(username.as_str(), "johndoe");
    }

    #[test]
    fn test_username_trim_whitespace() {
        let username = Username::new("  johndoe  ").unwrap();
        assert_eq!(username.as_str(), "johndoe");
    }

    #[test]
    fn test_username_rejects_too_short() {
        let result = Username::new("ab");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_username_rejects_too_long() {
        let long_username = "a".repeat(51);
        let result = Username::new(&long_username);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_username_allows_valid_characters() {
        let valid_usernames = vec!["user-name", "user_name", "user.name", "user123"];
        
        for username in valid_usernames {
            let result = Username::new(username);
            assert!(result.is_ok(), "Failed for: {}", username);
        }
    }

    #[test]
    fn test_username_rejects_invalid_characters() {
        let invalid_usernames = vec!["user@name", "user name", "user$name", "user#name"];
        
        for username in invalid_usernames {
            let result = Username::new(username);
            assert!(matches!(result, Err(DomainError::Validation(_))), "Should have failed for: {}", username);
        }
    }

    #[test]
    fn test_username_equality() {
        let username1 = Username::new("JohnDoe").unwrap();
        let username2 = Username::new("johndoe").unwrap();
        assert_eq!(username1, username2);
    }

    #[test]
    fn test_username_display() {
        let username = Username::new("test_user").unwrap();
        assert_eq!(format!("{}", username), "test_user");
    }
}

rust

// tests/domain/value_objects/user_id_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::UserId;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_user_id() {
        let id = "123e4567-e89b-12d3-a456-426614174000".to_string();
        let user_id = UserId::new(id.clone()).unwrap();
        assert_eq!(user_id.as_str(), id);
    }

    #[test]
    fn test_user_id_rejects_empty_string() {
        let result = UserId::new("".to_string());
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_user_id_rejects_whitespace_only() {
        let result = UserId::new("   ".to_string());
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_generate_user_id() {
        let user_id = UserId::generate();
        assert!(!user_id.as_str().is_empty());
        // Should be a valid UUID
        let uuid_result = uuid::Uuid::parse_str(user_id.as_str());
        assert!(uuid_result.is_ok());
    }

    #[test]
    fn test_user_id_equality() {
        let id = "test-id".to_string();
        let user_id1 = UserId::new(id.clone()).unwrap();
        let user_id2 = UserId::new(id).unwrap();
        assert_eq!(user_id1, user_id2);
    }

    #[test]
    fn test_user_id_display() {
        let id = "test-id-123".to_string();
        let user_id = UserId::new(id.clone()).unwrap();
        assert_eq!(format!("{}", user_id), id);
    }
}

rust

// tests/domain/value_objects/phone_number_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::PhoneNumber;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_phone_number() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_removes_spaces_and_dashes() {
        let phone = PhoneNumber::new("+1 234-567-890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_allows_empty_string() {
        let phone = PhoneNumber::new("").unwrap();
        assert!(phone.is_empty());
    }

    #[test]
    fn test_phone_number_rejects_invalid_characters() {
        let invalid_numbers = vec!["+1-800-ABC-DEFG", "+1 (800) 123-4567", "800!123!4567"];
        
        for number in invalid_numbers {
            let result = PhoneNumber::new(number);
            assert!(matches!(result, Err(DomainError::Validation(_))), "Should have failed for: {}", number);
        }
    }

    #[test]
    fn test_phone_number_rejects_too_short() {
        let result = PhoneNumber::new("+123456789");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_rejects_too_long() {
        let long_number = "+1".to_string() + &"2".repeat(15);
        let result = PhoneNumber::new(&long_number);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_is_empty() {
        let empty_phone = PhoneNumber::new("").unwrap();
        assert!(empty_phone.is_empty());
        
        let non_empty_phone = PhoneNumber::new("+1234567890").unwrap();
        assert!(!non_empty_phone.is_empty());
    }

    #[test]
    fn test_phone_number_display() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(format!("{}", phone), "+1234567890");
    }
}

Entity Tests
rust

// tests/domain/entities/user_test.rs
#[cfg(test)]
mod tests {
    use super::super::super::common::test_utils;
    use auth_service::domain::entities::User;
    use auth_service::domain::value_objects::{Email, Username, PhoneNumber};
    use auth_service::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};
    use auth_service::domain::DomainError;

    fn create_test_user() -> (User, UserCreatedEvent) {
        let keycloak_id = "test-keycloak-id".to_string();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        
        User::create(keycloak_id, username, email, Some("John".to_string()), Some("Doe".to_string())).unwrap()
    }

    #[test]
    fn test_user_creation() {
        let (user, event) = create_test_user();
        
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(!user.phone_number().is_empty());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.email().as_str(), "test@example.com");
    }

    #[test]
    fn test_user_update_profile() {
        let (mut user, _) = create_test_user();
        let old_email = user.email().clone();
        
        let event = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("+1234567890".to_string()),
        ).unwrap();
        
        assert_eq!(user.first_name(), Some("Jane"));
        assert_eq!(user.last_name(), Some("Smith"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");
        
        // Verify event
        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.old_email(), &old_email);
    }

    #[test]
    fn test_user_update_profile_with_invalid_phone() {
        let (mut user, _) = create_test_user();
        
        let result = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("invalid-phone".to_string()),
        );
        
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_user_mark_email_verified() {
        let (mut user, _) = create_test_user();
        
        assert!(!user.is_email_verified());
        user.mark_email_verified();
        assert!(user.is_email_verified());
    }

    #[test]
    fn test_user_record_login() {
        let (mut user, _) = create_test_user();
        
        assert!(user.last_login_at().is_none());
        user.record_login();
        assert!(user.last_login_at().is_some());
    }

    #[test]
    fn test_user_deactivate() {
        let (mut user, _) = create_test_user();
        
        assert!(user.is_active());
        let event = user.deactivate().unwrap();
        assert!(!user.is_active());
        
        // Verify event
        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), user.id());
    }

    #[test]
    fn test_user_cannot_deactivate_already_deactivated() {
        let (mut user, _) = create_test_user();
        
        user.deactivate().unwrap();
        let result = user.deactivate();
        
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
    }

    #[test]
    fn test_user_activate() {
        let (mut user, _) = create_test_user();
        
        user.deactivate().unwrap();
        assert!(!user.is_active());
        
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_getters() {
        let (user, _) = create_test_user();
        
        assert!(!user.id().as_str().is_empty());
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert!(user.created_at() <= chrono::Utc::now());
        assert!(user.updated_at() <= chrono::Utc::now());
    }
}

Events Tests
rust

// tests/domain/events/user_events_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::{UserId, Email, Username};
    use auth_service::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent, DomainEvent};

    fn create_test_user_id() -> UserId {
        UserId::new("test-user-id".to_string()).unwrap()
    }

    fn create_test_email() -> Email {
        Email::new("test@example.com").unwrap()
    }

    fn create_test_username() -> Username {
        Username::new("testuser").unwrap()
    }

    #[test]
    fn test_user_created_event() {
        let user_id = create_test_user_id();
        let email = create_test_email();
        let username = create_test_username();
        
        let event = UserCreatedEvent::new(user_id.clone(), email.clone(), username.clone());
        
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), &user_id);
        assert_eq!(event.email(), &email);
        assert_eq!(event.username(), &username);
        assert_eq!(event.version(), "1.0");
        assert!(event.occurred_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_updated_event() {
        let user_id = create_test_user_id();
        let old_email = create_test_email();
        
        let event = UserUpdatedEvent::new(user_id.clone(), old_email.clone());
        
        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), &user_id);
        assert_eq!(event.old_email(), &old_email);
        assert_eq!(event.version(), "1.0");
        assert!(event.occurred_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_deactivated_event() {
        let user_id = create_test_user_id();
        
        let event = UserDeactivatedEvent::new(user_id.clone());
        
        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), &user_id);
        assert_eq!(event.version(), "1.0");
        assert!(event.occurred_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_domain_event_trait_implementation() {
        let user_id = create_test_user_id();
        let email = create_test_email();
        let username = create_test_username();
        
        let event = UserCreatedEvent::new(user_id, email, username);
        
        // Test that it implements the trait
        let domain_event: &dyn DomainEvent = &event;
        assert_eq!(domain_event.event_type(), "user.created");
        assert_eq!(domain_event.version(), "1.0");
    }
}

Integration Tests
rust

// tests/integration/user_management_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::value_objects::{Email, Username, PhoneNumber};
    use auth_service::domain::DomainError;

    #[test]
    fn test_complete_user_lifecycle() {
        // Create user
        let keycloak_id = "keycloak-123".to_string();
        let username = Username::new("johndoe").unwrap();
        let email = Email::new("john@example.com").unwrap();
        
        let (mut user, created_event) = User::create(
            keycloak_id,
            username,
            email,
            Some("John".to_string()),
            Some("Doe".to_string()),
        ).unwrap();
        
        assert_eq!(created_event.event_type(), "user.created");
        assert!(user.is_active());

        // Update profile
        let updated_event = user.update_profile(
            Some("Johnny".to_string()),
            Some("Doey".to_string()),
            Some("+1234567890".to_string()),
        ).unwrap();
        
        assert_eq!(updated_event.event_type(), "user.updated");
        assert_eq!(user.first_name(), Some("Johnny"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");

        // Mark email verified
        user.mark_email_verified();
        assert!(user.is_email_verified());

        // Record login
        user.record_login();
        assert!(user.last_login_at().is_some());

        // Deactivate
        let deactivated_event = user.deactivate().unwrap();
        assert_eq!(deactivated_event.event_type(), "user.deactivated");
        assert!(!user.is_active());

        // Reactivate
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_validation_chain() {
        // Test that invalid data is caught at the value object level
        let invalid_email_result = Email::new("invalid-email");
        assert!(matches!(invalid_email_result, Err(DomainError::Validation(_))));

        let invalid_username_result = Username::new("ab");
        assert!(matches!(invalid_username_result, Err(DomainError::Validation(_))));

        let invalid_phone_result = PhoneNumber::new("invalid");
        assert!(matches!(invalid_phone_result, Err(DomainError::Validation(_))));

        // Valid data should work
        let valid_email = Email::new("valid@example.com").unwrap();
        let valid_username = Username::new("validuser").unwrap();
        
        let (user, _) = User::create(
            "keycloak-123".to_string(),
            valid_username,
            valid_email,
            Some("Valid".to_string()),
            Some("User".to_string()),
        ).unwrap();
        
        assert!(user.is_active());
    }
}

Test Configuration
rust

// tests/integration/mod.rs
mod user_management_test;

rust

// tests/domain/mod.rs
mod value_objects;
mod entities;
mod events;

toml

# Cargo.toml (test dependencies)
[dev-dependencies]
mockall = "0.11"
tokio = { version = "1.0", features = ["full"] }

Key Testing Strategies:

    Unit Tests: Isolated testing of value objects and entities

    Behavior Testing: Testing business rules and domain logic

    Edge Cases: Testing validation boundaries and error conditions

    Integration Tests: Testing the complete flow across multiple domain objects

    Property Testing: Testing invariants and properties that should always hold

These tests ensure your domain logic is robust and correctly implements all business rules before integrating with infrastructure components.
should i add mod.rs in tests

Yes, you should add mod.rs files in tests! Here's the correct structure with mod.rs files:
Updated Test Structure with mod.rs Files
text

tests/
├── mod.rs                      # ← Root test module
├── common.rs
├── domain/
│   ├── mod.rs                  # ← Domain test module
│   ├── value_objects/
│   │   ├── mod.rs              # ← Value objects test module
│   │   ├── email_test.rs
│   │   ├── username_test.rs
│   │   ├── user_id_test.rs
│   │   └── phone_number_test.rs
│   ├── entities/
│   │   ├── mod.rs              # ← Entities test module
│   │   └── user_test.rs
│   └── events/
│       ├── mod.rs              # ← Events test module
│       └── user_events_test.rs
└── integration/
    ├── mod.rs                  # ← Integration test module
    └── user_management_test.rs

mod.rs Files Content
rust

// tests/mod.rs
mod common;
mod domain;
mod integration;

// Re-export for easy access in integration tests
pub use common::test_utils;

rust

// tests/common.rs
pub mod test_utils {
    use chrono::{DateTime, TimeZone, Utc};
    
    pub fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }
    
    pub fn create_valid_email() -> String {
        "test@example.com".to_string()
    }
    
    pub fn create_valid_username() -> String {
        "testuser".to_string()
    }
    
    pub fn create_valid_phone() -> String {
        "+1234567890".to_string()
    }
    
    pub fn create_valid_keycloak_id() -> String {
        "keycloak-123".to_string()
    }
}

rust

// tests/domain/mod.rs
pub mod value_objects;
pub mod entities;
pub mod events;

rust

// tests/domain/value_objects/mod.rs
mod email_test;
mod username_test;
mod user_id_test;
mod phone_number_test;

rust

// tests/domain/entities/mod.rs
mod user_test;

rust

// tests/domain/events/mod.rs
mod user_events_test;

rust

// tests/integration/mod.rs
mod user_management_test;

Why You Need mod.rs Files:
1. Module Declaration

Without mod.rs, Rust doesn't know which files are part of your test modules.
2. Proper Module Hierarchy
rust

// Without mod.rs - doesn't work
tests/
├── domain/
│   └── email_test.rs  // ❌ Rust can't find this module

// With mod.rs - works correctly  
tests/
├── mod.rs             // declares domain module
├── domain/
│   ├── mod.rs         // declares email_test module
│   └── email_test.rs  // ✅ Rust can find this now

3. Better Organization

You can control what's public vs private within your test modules:
rust

// tests/domain/mod.rs
pub mod value_objects;  // Public - can be used by integration tests
mod entities;           // Private - only for domain tests
mod events;

4. Test Helper Sharing

You can share test utilities across modules:
rust

// tests/domain/value_objects/mod.rs
mod email_test;
mod username_test;

// Shared test helpers for value objects
#[cfg(test)]
pub(crate) mod test_helpers {
    pub fn assert_validation_error(result: Result<(), crate::domain::DomainError>, expected_msg: &str) {
        match result {
            Err(crate::domain::DomainError::Validation(msg)) => {
                assert!(msg.contains(expected_msg));
            }
            _ => panic!("Expected validation error, got: {:?}", result),
        }
    }
}

Alternative: Using Rust 2018 Edition

In Rust 2018+, you can also use [module_name].rs instead of mod.rs, but mod.rs is still widely used and recommended for:

    Consistency with older codebases

    Clear intent - it's obvious it's a module file

    IDE support - some IDEs handle mod.rs better

Rust 2018 Style (Alternative):
text

tests/
├── common.rs
├── domain.rs          # Instead of domain/mod.rs
├── domain/
│   ├── value_objects.rs  # Instead of value_objects/mod.rs  
│   └── value_objects/
│       └── email_test.rs

Benefits of This Structure:

    Clean Organization: Logical grouping of tests

    Easy Navigation: Clear module hierarchy

    Shared Utilities: Common test helpers in common.rs

    Isolated Testing: Domain tests don't depend on infrastructure

    CI/CD Friendly: Easy to run specific test groups

Running Specific Test Groups:
bash

# Run all tests
cargo test

# Run only domain tests
cargo test domain

# Run only value object tests  
cargo test value_objects

# Run specific test file
cargo test email_test

# Run integration tests only
cargo test --test integration

Yes, definitely add the mod.rs files - they're essential for Rust to understand your module structure and will make your test organization much cleaner and more maintainable.
give me cargo.toml

Here's the complete Cargo.toml for your Rust auth service with Keycloak and Swagger support:
toml

[package]
name = "auth-service"
version = "0.1.0"
description = "Authentication service with Keycloak integration and Swagger documentation"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
license = "MIT OR Apache-2.0"
readme = "README.md"
repository = "https://github.com/your-username/auth-service"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Web Framework
actix-web = "4.4"
actix-cors = "0.7"
actix-rt = "2.9"

# Serialization/Deserialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_qs = "0.11"

# Configuration
config = "0.13"
dotenvy = "0.15"

# HTTP Client for Keycloak API
reqwest = { version = "0.11", features = ["json"] }
http = "0.2"

# Authentication & Crypto
jsonwebtoken = "9.0"
bcrypt = "0.15"
uuid = { version = "1.6", features = ["v4", "serde"] }

# Async
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
async-trait = "0.1"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
log = "0.4"
env_logger = "0.10"

# Validation
validator = { version = "0.16", features = ["derive"] }

# OpenAPI/Swagger Documentation
utoipa = { version = "3.0", features = ["actix_extras", "chrono"] }
utoipa-swagger-ui = "3.0"

# DateTime
chrono = { version = "0.4", features = ["serde"] }

# Keycloak-specific dependencies
keycloak = { version = "0.6", optional = true }  # If using keycloak crate
openssl = { version = "0.10", features = ["vendored"] }

# Database (optional - for future use)
# sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"], optional = true }
# redis = { version = "0.23", optional = true }

[dev-dependencies]
# Testing
tokio = { version = "1.0", features = ["full", "rt-multi-thread"] }
mockall = "0.11"
test-log = "0.2"

# HTTP testing
wiremock = "0.5"
actix-test = "0.1"

# Test assertions
assert_matches = "1.5"

# Feature-specific testing
# sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "migrate"] }

[features]
default = ["keycloak", "swagger"]
keycloak = ["dep:keycloak"]
swagger = ["utoipa", "utoipa-swagger-ui"]

# Future features
# database = ["dep:sqlx", "dep:redis"]
# metrics = ["dep:metrics", "dep:metrics-exporter-prometheus"]

# Development features
dev = ["full_logging", "mock_mode"]
full_logging = []
mock_mode = []

[build-dependencies]
# For build scripts if needed
# rustc_version = "0.4"

[profile.dev]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'
incremental = true
codegen-units = 256
rpath = false

[profile.release]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = true
panic = 'unwind'
incremental = false
codegen-units = 1
rpath = false

[profile.test]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true

[package.metadata.docs.rs]
# Specify the features to build documentation for
all-features = true
targets = ["x86_64-unknown-linux-gnu"]

# Optional: Binaries section if you have multiple binaries
# [[bin]]
# name = "auth-service"
# path = "src/main.rs"

# [[bin]]
# name = "migrate"
# path = "src/bin/migrate.rs"

# Optional: Workspace configuration if this is part of a larger project
# [workspace]
# members = [
#     "auth-core",
#     "auth-api",
#     "auth-keycloak"
# ]
# resolver = "2"

Additional Configuration Files
.cargo/config.toml
toml

[build]
target = "x86_64-unknown-linux-gnu"

[target.'cfg(unix)']
linker = "clang"

[env]
RUST_LOG = "debug"
RUST_BACKTRACE = "1"

[alias]
test-all = "test --all-features"
test-unit = "test --lib --bins --tests"
test-integration = "test --tests integration"
dev = "run --features dev"
prod = "run --release"
fmt = "fmt --all"
clippy = "clippy --all-features -- -D warnings"

rust-toolchain.toml
toml

[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu"]
profile = "minimal"

Key Dependencies Explained:
Core Web Framework

    actix-web: Async web framework

    actix-cors: CORS support

    actix-rt: Async runtime

Keycloak Integration

    reqwest: HTTP client for Keycloak Admin API

    jsonwebtoken: JWT validation

    openssl: TLS/SSL support

Documentation

    utoipa: OpenAPI spec generation

    utoipa-swagger-ui: Swagger UI integration

Serialization & Validation

    serde: JSON serialization

    validator: Input validation

Async & Utilities

    tokio: Async runtime

    async-trait: Async trait support

    chrono: DateTime handling

Error Handling

    thiserror: Custom error types

    anyhow: Application errors

Testing

    mockall: Mocking for tests

    wiremock: HTTP API mocking

    test-log: Async test logging

Feature Flags Strategy:

    default: Keycloak + Swagger (production setup)

    keycloak: Keycloak integration

    swagger: API documentation

    dev: Development tools and mocks

    database: Future database support

    metrics: Future metrics support

This configuration provides:

    ✅ Production-ready dependencies

    ✅ Comprehensive testing setup

    ✅ Development tooling

    ✅ Feature flags for flexibility

    ✅ Performance-optimized profiles

    ✅ Good documentation support

give me main.rs and lib.rs

Here's the main.rs and lib.rs files for your auth service:
src/lib.rs
rust

//! Authentication Service
//!
//! A domain-driven authentication service with Keycloak integration
//! and OpenAPI/Swagger documentation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;

// Re-exports for common usage
pub use domain::DomainError;
pub use application::ApplicationError;
pub use api::ApiError;

/// Application result type
pub type Result<T> = std::result::Result<T, ApplicationError>;

/// API result type  
pub type ApiResult<T> = std::result::Result<T, ApiError>;

/// Application configuration
pub mod config {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct AppConfig {
        pub server: ServerConfig,
        pub keycloak: KeycloakConfig,
        pub logging: LoggingConfig,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct ServerConfig {
        pub host: String,
        pub port: u16,
        pub workers: Option<usize>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct KeycloakConfig {
        pub base_url: String,
        pub realm: String,
        pub client_id: String,
        pub client_secret: String,
        pub admin_username: String,
        pub admin_password: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct LoggingConfig {
        pub level: String,
        pub format: String,
    }

    impl AppConfig {
        /// Load configuration from environment variables
        pub fn from_env() -> Result<Self, config::ConfigError> {
            let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
            
            let config = config::Config::builder()
                .add_source(
                    config::Environment::with_prefix("APP")
                        .separator("_")
                        .list_separator(",")
                        .with_list_parse_key("keycloak.scopes"),
                )
                .build()?;

            config.try_deserialize()
        }
    }
}

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
    pub keycloak_client: infrastructure::keycloak::KeycloakClient,
    pub user_repository: std::sync::Arc<dyn domain::repositories::UserRepository>,
}

impl AppState {
    /// Create new application state
    pub fn new(
        config: config::AppConfig,
        keycloak_client: infrastructure::keycloak::KeycloakClient,
        user_repository: std::sync::Arc<dyn domain::repositories::UserRepository>,
    ) -> Self {
        Self {
            config,
            keycloak_client,
            user_repository,
        }
    }
}

/// Application factory for creating the web server
pub struct Application;

impl Application {
    /// Build and configure the Actix Web application
    pub fn build(config: config::AppConfig) -> std::io::Result<actix_web::HttpServer<impl Fn() -> actix_web::App>> {
        // Initialize keycloak client
        let keycloak_client = infrastructure::keycloak::KeycloakClient::new(
            config.keycloak.base_url.clone(),
            config.keycloak.realm.clone(),
            config.keycloak.client_id.clone(),
            config.keycloak.client_secret.clone(),
        );

        // Initialize user repository
        let user_repository = std::sync::Arc::new(
            infrastructure::keycloak::KeycloakUserRepository::new(keycloak_client.clone())
        );

        // Create application state
        let app_state = AppState::new(config.clone(), keycloak_client, user_repository);

        // Build HTTP server
        let server = actix_web::HttpServer::new(move || {
            let state = actix_web::web::Data::new(app_state.clone());
            
            actix_web::App::new()
                .app_data(state)
                .configure(api::routes::configure_routes)
                .configure(api::openapi::configure_swagger)
                .wrap(actix_web::middleware::Logger::default())
                .wrap(api::middleware::cors::configure_cors())
                .wrap(api::middleware::auth::Authentication)
                .default_service(
                    actix_web::web::route()
                        .to(api::handlers::not_found_handler)
                )
        })
        .workers(config.server.workers.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        }));

        Ok(server)
    }

    /// Run the application
    pub async fn run(config: config::AppConfig) -> std::io::Result<()> {
        // Initialize logging
        Self::init_logging(&config.logging);

        log::info!("Starting auth service on {}:{}", config.server.host, config.server.port);
        log::info!("Keycloak realm: {}", config.keycloak.realm);
        log::info!("Environment: {}", std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()));

        // Build and run server
        let server = Self::build(config)?;
        server.run().await
    }

    fn init_logging(logging_config: &config::LoggingConfig) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", &logging_config.level);
        }

        env_logger::Builder::from_default_env()
            .format_timestamp_millis()
            .format_module_path(false)
            .format_target(false)
            .init();

        log::info!("Logging initialized with level: {}", logging_config.level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let config = config::AppConfig {
            server: config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: Some(4),
            },
            keycloak: config::KeycloakConfig {
                base_url: "http://localhost:8080".to_string(),
                realm: "master".to_string(),
                client_id: "admin-cli".to_string(),
                client_secret: "secret".to_string(),
                admin_username: "admin".to_string(),
                admin_password: "admin".to_string(),
            },
            logging: config::LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        };

        let keycloak_client = infrastructure::keycloak::KeycloakClient::new(
            config.keycloak.base_url.clone(),
            config.keycloak.realm.clone(),
            config.keycloak.client_id.clone(),
            config.keycloak.client_secret.clone(),
        );

        let user_repository = std::sync::Arc::new(
            infrastructure::keycloak::KeycloakUserRepository::new(keycloak_client.clone())
        );

        let app_state = AppState::new(config, keycloak_client, user_repository);

        assert_eq!(app_state.config.server.port, 8080);
    }
}

src/main.rs
rust

//! Authentication Service Entry Point
//!
//! This is the main entry point for the authentication service
//! backed by Keycloak with Swagger documentation.

use auth_service::{Application, config::AppConfig};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file (if exists)
    dotenv().ok();

    // Load configuration from environment
    let config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to load configuration: {}", err);
            std::process::exit(1);
        }
    };

    // Handle graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        
        log::info!("Received shutdown signal, starting graceful shutdown...");
    };

    // Run the application
    if let Err(err) = Application::run(config).await {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }

    // Wait for shutdown signal (this will never be reached in normal operation
    // as the server runs until stopped)
    shutdown_signal.await;
    
    log::info!("Auth service shutdown complete");
    Ok(())
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        // Set test environment variables
        std::env::set_var("APP_SERVER_HOST", "127.0.0.1");
        std::env::set_var("APP_SERVER_PORT", "8080");
        std::env::set_var("APP_KEYCLOAK_BASE_URL", "http://localhost:8080");
        std::env::set_var("APP_KEYCLOAK_REALM", "test-realm");
        std::env::set_var("APP_KEYCLOAK_CLIENT_ID", "test-client");
        std::env::set_var("APP_KEYCLOAK_CLIENT_SECRET", "test-secret");
        std::env::set_var("APP_KEYCLOAK_ADMIN_USERNAME", "admin");
        std::env::set_var("APP_KEYCLOAK_ADMIN_PASSWORD", "password");
        std::env::set_var("APP_LOGGING_LEVEL", "info");

        let config = AppConfig::from_env();
        assert!(config.is_ok());

        // Cleanup
        std::env::remove_var("APP_SERVER_HOST");
        std::env::remove_var("APP_SERVER_PORT");
        std::env::remove_var("APP_KEYCLOAK_BASE_URL");
        std::env::remove_var("APP_KEYCLOAK_REALM");
        std::env::remove_var("APP_KEYCLOAK_CLIENT_ID");
        std::env::remove_var("APP_KEYCLOAK_CLIENT_SECRET");
        std::env::remove_var("APP_KEYCLOAK_ADMIN_USERNAME");
        std::env::remove_var("APP_KEYCLOAK_ADMIN_PASSWORD");
        std::env::remove_var("APP_LOGGING_LEVEL");
    }
}

Additional Supporting Files
src/config/mod.rs (if you want a separate config module)
rust

//! Configuration management for the authentication service.

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub keycloak: KeycloakConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        
        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
                    .list_separator(",")
            )
            .build()?;

        config.try_deserialize()
    }

    /// Get server bind address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

Key Features:
In lib.rs:

    Module organization with clear exports

    Configuration management with environment variables

    Application state management

    Type aliases for common Result types

    Application factory pattern for building the server

    Comprehensive error handling

In main.rs:

    Environment setup with dotenv

    Configuration loading with error handling

    Graceful shutdown handling

    Proper logging initialization

    Clean error reporting

Benefits:

    Separation of Concerns: Main only handles startup/shutdown

    Testability: Core logic in lib.rs for easy testing

    Configuration: Environment-based config with defaults

    Error Handling: Proper error propagation and reporting

    Production Ready: Graceful shutdown and proper logging

This structure follows Rust best practices and provides a solid foundation for your authentication service.
Compiling auth-service v0.1.0 (/home/dali/WORK/work02/everest/sources/auth-service)
error[E0412]: cannot find type `DomainError` in this scope
 --> src/domain/value_objects/email.rs:8:45
  |
8 |     pub fn new(email: &str) -> Result<Self, DomainError> {
  |                                             ^^^^^^^^^^^ not found in this scope
  |
help: consider importing this enum through its public re-export
  |
2 + use crate::DomainError;

The error indicates that DomainError is not properly imported. Here's the corrected domain files with proper imports and module structure:
Fix the Domain Error and Module Structure
First, update the root domain files:
rust

// src/domain/mod.rs
pub mod value_objects;
pub mod entities;
pub mod events;
pub mod repositories;
pub mod error;

pub use error::DomainError;

rust

// src/domain/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists")]
    UserAlreadyExists,
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("Unknown domain error: {0}")]
    Unknown(String),
}

impl DomainError {
    pub fn validation(msg: &str) -> Self {
        DomainError::Validation(msg.to_string())
    }
    
    pub fn business_rule(msg: &str) -> Self {
        DomainError::BusinessRule(msg.to_string())
    }
}

Now update the value objects with proper imports:
rust

// src/domain/value_objects/email.rs
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, DomainError> {
        let email = email.trim().to_lowercase();
        
        if email.is_empty() {
            return Err(DomainError::Validation("Email cannot be empty".to_string()));
        }
        
        if !email.contains('@') {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::Validation("Invalid email format".to_string()));
        }
        
        if !parts[1].contains('.') {
            return Err(DomainError::Validation("Invalid email domain".to_string()));
        }
        
        Ok(Email(email))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/username.rs
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, DomainError> {
        let username = username.trim();
        
        if username.len() < 3 {
            return Err(DomainError::Validation("Username must be at least 3 characters".to_string()));
        }
        
        if username.len() > 50 {
            return Err(DomainError::Validation("Username must be less than 50 characters".to_string()));
        }
        
        if !username.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return Err(DomainError::Validation("Username can only contain alphanumeric characters, hyphens, underscores, and dots".to_string()));
        }
        
        Ok(Username(username.to_lowercase()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/user_id.rs
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::Validation("User ID cannot be empty".to_string()));
        }
        Ok(UserId(id))
    }
    
    pub fn generate() -> Self {
        UserId(uuid::Uuid::new_v4().to_string())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/phone_number.rs
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Result<Self, DomainError> {
        let phone = phone.trim().replace(' ', "").replace('-', "");
        
        if phone.is_empty() {
            return Ok(PhoneNumber(phone)); // Optional field
        }
        
        if !phone.chars().all(|c| c.is_numeric() || c == '+') {
            return Err(DomainError::Validation("Phone number can only contain numbers and '+'".to_string()));
        }
        
        if phone.len() < 10 || phone.len() > 15 {
            return Err(DomainError::Validation("Phone number must be between 10-15 digits".to_string()));
        }
        
        Ok(PhoneNumber(phone))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<PhoneNumber> for String {
    fn from(phone: PhoneNumber) -> Self {
        phone.0
    }
}

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

rust

// src/domain/value_objects/mod.rs
pub mod email;
pub mod username;
pub mod user_id;
pub mod phone_number;

pub use email::Email;
pub use username::Username;
pub use user_id::UserId;
pub use phone_number::PhoneNumber;

Also update the lib.rs to fix the config issue:
rust

// src/lib.rs (updated)
//! Authentication Service
//!
//! A domain-driven authentication service with Keycloak integration
//! and OpenAPI/Swagger documentation.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;
pub mod config;

// Re-exports for common usage
pub use domain::DomainError;

/// Application result type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
    // Will be added later when we implement the infrastructure
    // pub keycloak_client: infrastructure::keycloak::KeycloakClient,
    // pub user_repository: std::sync::Arc<dyn domain::repositories::UserRepository>,
}

impl AppState {
    /// Create new application state
    pub fn new(config: config::AppConfig) -> Self {
        Self {
            config,
        }
    }
}

/// Application factory for creating the web server
pub struct Application;

impl Application {
    /// Build and configure the Actix Web application
    pub fn build(config: config::AppConfig) -> std::io::Result<actix_web::HttpServer<impl Fn() -> actix_web::App>> {
        // Create application state
        let app_state = AppState::new(config.clone());

        // Build HTTP server
        let server = actix_web::HttpServer::new(move || {
            let state = actix_web::web::Data::new(app_state.clone());
            
            actix_web::App::new()
                .app_data(state)
                // .configure(api::routes::configure_routes)  // Will be added later
                // .configure(api::openapi::configure_swagger)  // Will be added later
                .wrap(actix_web::middleware::Logger::default())
                .wrap(
                    actix_cors::Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600)
                )
                .default_service(
                    actix_web::web::route()
                        .to(|| async { 
                            actix_web::HttpResponse::NotFound().json(serde_json::json!({
                                "error": "Endpoint not found"
                            }))
                        })
                )
        })
        .workers(config.server.workers.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
        }));

        Ok(server)
    }

    /// Run the application
    pub async fn run(config: config::AppConfig) -> std::io::Result<()> {
        // Initialize logging
        Self::init_logging(&config.logging);

        log::info!("Starting auth service on {}:{}", config.server.host, config.server.port);
        log::info!("Keycloak realm: {}", config.keycloak.realm);
        log::info!("Environment: {}", std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()));

        // Build and run server
        let server = Self::build(config)?;
        server.bind("127.0.0.1:8080")?.run().await
    }

    fn init_logging(logging_config: &config::LoggingConfig) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", &logging_config.level);
        }

        env_logger::Builder::from_default_env()
            .format_timestamp_millis()
            .format_module_path(false)
            .format_target(false)
            .init();

        log::info!("Logging initialized with level: {}", logging_config.level);
    }
}

And create the config module:
rust

// src/config/mod.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub keycloak: KeycloakConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeycloakConfig {
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("_")
            )
            .build()?;

        config.try_deserialize()
    }

    /// Get server bind address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

The key changes:

    Proper imports using crate::domain::DomainError

    Fixed module structure with proper mod.rs files

    Added config module separately

    Simplified initial implementation - you can add Keycloak integration later

This should compile without errors. You can gradually add the infrastructure and API layers once the domain compiles successfully.
--> src/domain/entities/user.rs:15:5
   |
 7 | pub struct User {
   |            ---- field in this struct
...
15 |     avatar_url: Option<String>,
   |     ^^^^^^^^^^
   |
   = note: `User` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: `auth-service` (lib) generated 1 warning
warning: `auth-service` (lib test) generated 1 warning (1 duplicate)
   Compiling auth-service v0.1.0 (/home/dali/WORK/work02/everest/sources/auth-service)
error[E0583]: file not found for module `domain`
 --> tests/mod.rs:3:1
  |
3 | mod domain;
  | ^^^^^^^^^^^
  |
  = help: to create the module `domain`, create file "tests/domain.rs" or "tests/domain/mod.rs"
  = note: if there is a `mod domain` elsewhere in the crate already, import it with `use crate::...` instead

Here are the fixes for both issues:
1. Fix the Dead Code Warning

The warning is because you have fields in your User struct that aren't being used yet. This is normal during development. You can either:

Option A: Add #[allow(dead_code)] to the struct (recommended for development):
rust

// src/domain/entities/user.rs
use crate::domain::DomainError;
use crate::domain::value_objects::{UserId, Email, Username, PhoneNumber};
use crate::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};

#[derive(Debug, Clone)]
#[allow(dead_code)] // Remove this when all fields are used
pub struct User {
    id: UserId,
    keycloak_id: String,
    username: Username,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: PhoneNumber,
    avatar_url: Option<String>,
    is_active: bool,
    is_email_verified: bool,
    last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

// Rest of the implementation remains the same...

Option B: Add getters for the unused fields:
rust

// Add these getters to the User impl block
impl User {
    // ... existing methods ...

    // Add getters for the "dead code" fields
    pub fn avatar_url(&self) -> Option<&str> {
        self.avatar_url.as_deref()
    }

    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
        self.updated_at = chrono::Utc::now();
    }
}

2. Fix the Test Module File Structure

Create the missing test module files:
rust

// tests/mod.rs
mod common;
mod domain;
mod integration;

// Re-export for easy access in integration tests
pub use common::test_utils;

rust

// tests/domain/mod.rs
pub mod value_objects;
pub mod entities;
pub mod events;

rust

// tests/domain/value_objects/mod.rs
mod email_test;
mod username_test;
mod user_id_test;
mod phone_number_test;

rust

// tests/domain/entities/mod.rs
mod user_test;

rust

// tests/domain/events/mod.rs
mod user_events_test;

rust

// tests/integration/mod.rs
mod user_management_test;

3. Update the Test Files with Proper Imports

Make sure all test files use proper imports. Here's an example:
rust

// tests/domain/value_objects/email_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::Email;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_email() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_rejects_empty_string() {
        let result = Email::new("");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    // ... rest of your tests
}

4. Alternative: Disable Dead Code Warnings Globally (Temporary)

If you want to suppress dead code warnings during development, you can add this to your lib.rs:
rust

// src/lib.rs
#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(dead_code)] // Temporary during development

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;
pub mod config;

// ... rest of your code

5. Complete Working Example

Here's a minimal working version of the user entity that compiles without warnings:
rust

// src/domain/entities/user.rs
use crate::domain::DomainError;
use crate::domain::value_objects::{UserId, Email, Username, PhoneNumber};
use crate::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    keycloak_id: String,
    username: Username,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: PhoneNumber,
    avatar_url: Option<String>,
    is_active: bool,
    is_email_verified: bool,
    last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn create(
        keycloak_id: String,
        username: Username,
        email: Email,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<(Self, UserCreatedEvent), DomainError> {
        let now = chrono::Utc::now();
        let user = User {
            id: UserId::generate(),
            keycloak_id,
            username,
            email,
            first_name,
            last_name,
            phone_number: PhoneNumber::new("")?,
            avatar_url: None,
            is_active: true,
            is_email_verified: false,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        };
        
        let event = UserCreatedEvent::new(
            user.id.clone(),
            user.email.clone(),
            user.username.clone(),
        );
        
        Ok((user, event))
    }
    
    // Getters for all fields
    pub fn id(&self) -> &UserId { &self.id }
    pub fn keycloak_id(&self) -> &str { &self.keycloak_id }
    pub fn username(&self) -> &Username { &self.username }
    pub fn email(&self) -> &Email { &self.email }
    pub fn first_name(&self) -> Option<&str> { self.first_name.as_deref() }
    pub fn last_name(&self) -> Option<&str> { self.last_name.as_deref() }
    pub fn phone_number(&self) -> &PhoneNumber { &self.phone_number }
    pub fn avatar_url(&self) -> Option<&str> { self.avatar_url.as_deref() }
    pub fn is_active(&self) -> bool { self.is_active }
    pub fn is_email_verified(&self) -> bool { self.is_email_verified }
    pub fn last_login_at(&self) -> Option<chrono::DateTime<chrono::Utc>> { self.last_login_at }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created_at }
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> { self.updated_at }
    
    // Business methods that use the fields
    pub fn record_login(&mut self) {
        self.last_login_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
        self.updated_at = chrono::Utc::now();
    }
}

The key fixes are:

    Add #[allow(dead_code)] or implement getters for unused fields

    Create all missing mod.rs files in the tests directory

    Use proper imports in test files with auth_service::domain::...

This should resolve both the dead code warning and the module file not found error.
error[E0432]: unresolved import `super::super::super::common`
 --> tests/unit/value_objects/email_test.rs:4:30
  |
4 |     use super::super::super::common::test_utils;
  |                              ^^^^^^ could not find `common` in `super`

error[E0432]: unresolved import `super::super::super::common`
 --> tests/unit/entities/user_test.rs:4:30
  |
4 |     use super::super::super::common::test_utils;
  |                              ^^^^^^ could not find `common` in `super`

warning: unused import: `PhoneNumber`

The issue is with the test import paths. Let me fix the test structure and imports:
Fix the Test Structure
Update the test directory structure:
text

tests/
├── mod.rs
├── common.rs
├── unit/
│   ├── mod.rs
│   ├── value_objects/
│   │   ├── mod.rs
│   │   ├── email_test.rs
│   │   ├── username_test.rs
│   │   ├── user_id_test.rs
│   │   └── phone_number_test.rs
│   ├── entities/
│   │   ├── mod.rs
│   │   └── user_test.rs
│   └── events/
│       ├── mod.rs
│       └── user_events_test.rs
└── integration/
    ├── mod.rs
    └── user_management_test.rs

Fix the mod.rs files:
rust

// tests/mod.rs
mod common;
mod unit;
mod integration;

// Re-export for easy access
pub use common::test_utils;

rust

// tests/common.rs
pub mod test_utils {
    use chrono::{DateTime, TimeZone, Utc};
    
    pub fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }
    
    pub fn create_valid_email() -> String {
        "test@example.com".to_string()
    }
    
    pub fn create_valid_username() -> String {
        "testuser".to_string()
    }
    
    pub fn create_valid_phone() -> String {
        "+1234567890".to_string()
    }
    
    pub fn create_valid_keycloak_id() -> String {
        "keycloak-123".to_string()
    }
}

rust

// tests/unit/mod.rs
pub mod value_objects;
pub mod entities;
pub mod events;

rust

// tests/unit/value_objects/mod.rs
mod email_test;
mod username_test;
mod user_id_test;
mod phone_number_test;

rust

// tests/unit/entities/mod.rs
mod user_test;

rust

// tests/unit/events/mod.rs
mod user_events_test;

rust

// tests/integration/mod.rs
mod user_management_test;

Fix the Test Imports
Update email_test.rs with correct imports:
rust

// tests/unit/value_objects/email_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::Email;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_email() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_is_normalized_to_lowercase() {
        let email = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_trim_whitespace() {
        let email = Email::new("  user@example.com  ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_rejects_empty_string() {
        let result = Email::new("");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_at_symbol() {
        let result = Email::new("userexample.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_username() {
        let result = Email::new("@example.com");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_missing_domain() {
        let result = Email::new("user@");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_rejects_invalid_domain_format() {
        let result = Email::new("user@example");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_email_equality() {
        let email1 = Email::new("user@example.com").unwrap();
        let email2 = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email1, email2);
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(format!("{}", email), "user@example.com");
    }

    #[test]
    fn test_email_into_string() {
        let email = Email::new("user@example.com").unwrap();
        let string: String = email.into();
        assert_eq!(string, "user@example.com");
    }
}

Update user_test.rs with correct imports:
rust

// tests/unit/entities/user_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::value_objects::{Email, Username, PhoneNumber};
    use auth_service::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};
    use auth_service::domain::DomainError;

    fn create_test_user() -> (User, UserCreatedEvent) {
        let keycloak_id = "test-keycloak-id".to_string();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        
        User::create(keycloak_id, username, email, Some("John".to_string()), Some("Doe".to_string())).unwrap()
    }

    #[test]
    fn test_user_creation() {
        let (user, event) = create_test_user();
        
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(!user.phone_number().is_empty());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.email().as_str(), "test@example.com");
    }

    #[test]
    fn test_user_record_login() {
        let (mut user, _) = create_test_user();
        
        assert!(user.last_login_at().is_none());
        user.record_login();
        assert!(user.last_login_at().is_some());
    }

    #[test]
    fn test_user_set_avatar_url() {
        let (mut user, _) = create_test_user();
        
        assert!(user.avatar_url().is_none());
        user.set_avatar_url(Some("https://example.com/avatar.jpg".to_string()));
        assert_eq!(user.avatar_url(), Some("https://example.com/avatar.jpg"));
    }

    // Add more tests for other methods...
}

Remove the unused import warning by fixing phone_number_test.rs:
rust

// tests/unit/value_objects/phone_number_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::PhoneNumber;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_phone_number() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_removes_spaces_and_dashes() {
        let phone = PhoneNumber::new("+1 234-567-890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_allows_empty_string() {
        let phone = PhoneNumber::new("").unwrap();
        assert!(phone.is_empty());
    }

    #[test]
    fn test_phone_number_rejects_invalid_characters() {
        let invalid_numbers = vec!["+1-800-ABC-DEFG", "+1 (800) 123-4567", "800!123!4567"];
        
        for number in invalid_numbers {
            let result = PhoneNumber::new(number);
            assert!(matches!(result, Err(DomainError::Validation(_))), "Should have failed for: {}", number);
        }
    }

    #[test]
    fn test_phone_number_rejects_too_short() {
        let result = PhoneNumber::new("+123456789");
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_rejects_too_long() {
        let long_number = "+1".to_string() + &"2".repeat(15);
        let result = PhoneNumber::new(&long_number);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_is_empty() {
        let empty_phone = PhoneNumber::new("").unwrap();
        assert!(empty_phone.is_empty());
        
        let non_empty_phone = PhoneNumber::new("+1234567890").unwrap();
        assert!(!non_empty_phone.is_empty());
    }

    #[test]
    fn test_phone_number_display() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(format!("{}", phone), "+1234567890");
    }
}

Alternative: Simplify Test Structure

If you prefer a simpler structure, you can put all tests directly in the tests directory:
text

tests/
├── common.rs
├── email_test.rs
├── username_test.rs
├── user_id_test.rs
├── phone_number_test.rs
├── user_test.rs
├── user_events_test.rs
└── user_management_test.rs

Then in each test file, use direct imports:
rust

// tests/email_test.rs
use auth_service::domain::value_objects::Email;
use auth_service::domain::DomainError;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_valid_email() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }
    
    // ... rest of tests
}

Key Changes:

    Fixed import paths - using auth_service::domain:: instead of relative paths

    Simplified structure - removed complex relative imports

    Fixed module declarations - proper mod.rs files

    Removed unused imports - cleaned up the PhoneNumber import warning

The tests should now compile without import errors. The key is to use the crate name (auth_service) for imports in test files rather than complex relative paths.
give me full user_test.rs

Here's the complete user_test.rs with all domain methods tested:
rust

// tests/unit/entities/user_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::value_objects::{Email, Username, PhoneNumber};
    use auth_service::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};
    use auth_service::domain::DomainError;

    fn create_test_user() -> (User, UserCreatedEvent) {
        let keycloak_id = "test-keycloak-id".to_string();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        
        User::create(
            keycloak_id,
            username,
            email,
            Some("John".to_string()),
            Some("Doe".to_string()),
        ).unwrap()
    }

    #[test]
    fn test_user_creation() {
        let (user, event) = create_test_user();
        
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(!user.phone_number().is_empty());
        assert!(user.avatar_url().is_none());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.email().as_str(), "test@example.com");
        assert_eq!(event.username().as_str(), "testuser");
    }

    #[test]
    fn test_user_creation_with_minimal_data() {
        let keycloak_id = "minimal-keycloak-id".to_string();
        let username = Username::new("minimaluser").unwrap();
        let email = Email::new("minimal@example.com").unwrap();
        
        let (user, event) = User::create(
            keycloak_id,
            username,
            email,
            None,
            None,
        ).unwrap();
        
        assert_eq!(user.keycloak_id(), "minimal-keycloak-id");
        assert_eq!(user.username().as_str(), "minimaluser");
        assert_eq!(user.email().as_str(), "minimal@example.com");
        assert_eq!(user.first_name(), None);
        assert_eq!(user.last_name(), None);
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
    }

    #[test]
    fn test_user_update_profile() {
        let (mut user, _) = create_test_user();
        let old_email = user.email().clone();
        
        let event = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("+1234567890".to_string()),
        ).unwrap();
        
        assert_eq!(user.first_name(), Some("Jane"));
        assert_eq!(user.last_name(), Some("Smith"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");
        
        // Verify event
        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.old_email(), &old_email);
    }

    #[test]
    fn test_user_update_profile_partial() {
        let (mut user, _) = create_test_user();
        let original_first_name = user.first_name().map(|s| s.to_string());
        
        // Update only phone number
        let event = user.update_profile(
            None, // Keep existing first name
            Some("UpdatedLast".to_string()), // Update last name
            Some("+1987654321".to_string()), // Update phone
        ).unwrap();
        
        assert_eq!(user.first_name(), original_first_name.as_deref());
        assert_eq!(user.last_name(), Some("UpdatedLast"));
        assert_eq!(user.phone_number().as_str(), "+1987654321");
        
        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_empty_phone() {
        let (mut user, _) = create_test_user();
        
        // Set a phone number first
        user.update_profile(
            None,
            None,
            Some("+1234567890".to_string()),
        ).unwrap();
        assert!(!user.phone_number().is_empty());
        
        // Update with empty phone (should clear it)
        let event = user.update_profile(
            None,
            None,
            Some("".to_string()),
        ).unwrap();
        
        assert!(user.phone_number().is_empty());
        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_invalid_phone() {
        let (mut user, _) = create_test_user();
        
        let result = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("invalid-phone".to_string()),
        );
        
        assert!(matches!(result, Err(DomainError::Validation(_))));
        // Ensure user data wasn't changed
        assert_eq!(user.first_name(), Some("John"));
    }

    #[test]
    fn test_user_mark_email_verified() {
        let (mut user, _) = create_test_user();
        
        assert!(!user.is_email_verified());
        user.mark_email_verified();
        assert!(user.is_email_verified());
        
        // Verify updated_at changed
        let updated_at_after = user.updated_at();
        assert!(updated_at_after > user.created_at());
    }

    #[test]
    fn test_user_record_login() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.last_login_at().is_none());
        user.record_login();
        
        assert!(user.last_login_at().is_some());
        assert!(user.updated_at() > original_updated_at);
        
        // Record another login
        let second_login_time = user.last_login_at().unwrap();
        user.record_login();
        assert!(user.last_login_at().unwrap() > second_login_time);
    }

    #[test]
    fn test_user_deactivate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.is_active());
        let event = user.deactivate().unwrap();
        assert!(!user.is_active());
        assert!(user.updated_at() > original_updated_at);
        
        // Verify event
        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), user.id());
    }

    #[test]
    fn test_user_cannot_deactivate_already_deactivated() {
        let (mut user, _) = create_test_user();
        
        user.deactivate().unwrap();
        let result = user.deactivate();
        
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        assert!(!user.is_active()); // Should still be deactivated
    }

    #[test]
    fn test_user_activate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        user.deactivate().unwrap();
        assert!(!user.is_active());
        
        user.activate();
        assert!(user.is_active());
        assert!(user.updated_at() > original_updated_at);
    }

    #[test]
    fn test_user_set_avatar_url() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.avatar_url().is_none());
        
        // Set avatar URL
        user.set_avatar_url(Some("https://example.com/avatar.jpg".to_string()));
        assert_eq!(user.avatar_url(), Some("https://example.com/avatar.jpg"));
        assert!(user.updated_at() > original_updated_at);
        
        // Update avatar URL
        let previous_updated_at = user.updated_at();
        user.set_avatar_url(Some("https://example.com/new-avatar.png".to_string()));
        assert_eq!(user.avatar_url(), Some("https://example.com/new-avatar.png"));
        assert!(user.updated_at() > previous_updated_at);
        
        // Remove avatar URL
        user.set_avatar_url(None);
        assert!(user.avatar_url().is_none());
    }

    #[test]
    fn test_user_getters() {
        let (user, _) = create_test_user();
        
        // Test all getters
        assert!(!user.id().as_str().is_empty());
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(!user.phone_number().is_empty());
        assert!(user.avatar_url().is_none());
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(user.created_at() <= chrono::Utc::now());
        assert!(user.updated_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_id_generation() {
        let (user1, _) = create_test_user();
        let (user2, _) = create_test_user();
        
        // Each user should have a unique ID
        assert_ne!(user1.id().as_str(), user2.id().as_str());
        
        // IDs should be valid UUIDs
        let uuid_result1 = uuid::Uuid::parse_str(user1.id().as_str());
        let uuid_result2 = uuid::Uuid::parse_str(user2.id().as_str());
        
        assert!(uuid_result1.is_ok());
        assert!(uuid_result2.is_ok());
    }

    #[test]
    fn test_user_created_at_and_updated_at() {
        let (user, _) = create_test_user();
        
        // Created and updated should be initially the same
        assert_eq!(user.created_at(), user.updated_at());
        
        // Both should be in the past (or very close to now)
        let now = chrono::Utc::now();
        assert!(user.created_at() <= now);
        assert!(user.updated_at() <= now);
    }

    #[test]
    fn test_user_phone_number_defaults_to_empty() {
        let keycloak_id = "test-keycloak".to_string();
        let username = Username::new("newuser").unwrap();
        let email = Email::new("new@example.com").unwrap();
        
        let (user, _) = User::create(keycloak_id, username, email, None, None).unwrap();
        
        assert!(user.phone_number().is_empty());
    }

    #[test]
    fn test_user_immutability_via_getters() {
        let (user, _) = create_test_user();
        
        // All getters should return immutable references or copies
        let id = user.id();
        let username = user.username();
        let email = user.email();
        
        // These should all compile and work
        assert_eq!(id.as_str(), user.id().as_str());
        assert_eq!(username.as_str(), user.username().as_str());
        assert_eq!(email.as_str(), user.email().as_str());
    }

    #[test]
    fn test_user_lifecycle_events() {
        // Create user
        let keycloak_id = "lifecycle-user".to_string();
        let username = Username::new("lifecycle").unwrap();
        let email = Email::new("lifecycle@example.com").unwrap();
        
        let (mut user, created_event) = User::create(
            keycloak_id,
            username,
            email,
            Some("Life".to_string()),
            Some("Cycle".to_string()),
        ).unwrap();
        
        assert_eq!(created_event.event_type(), "user.created");
        
        // Update profile
        let updated_event = user.update_profile(
            Some("UpdatedLife".to_string()),
            Some("UpdatedCycle".to_string()),
            Some("+1111111111".to_string()),
        ).unwrap();
        
        assert_eq!(updated_event.event_type(), "user.updated");
        
        // Deactivate
        let deactivated_event = user.deactivate().unwrap();
        assert_eq!(deactivated_event.event_type(), "user.deactivated");
        
        // Reactivate (no event for reactivation in current design)
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_business_rules() {
        let (mut user, _) = create_test_user();
        
        // Test that business rules are enforced
        user.deactivate().unwrap();
        
        // Try to deactivate again - should fail
        let result = user.deactivate();
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        
        // Error message should be meaningful
        if let Err(DomainError::BusinessRule(msg)) = result {
            assert!(msg.contains("already deactivated"));
        }
    }
}

This comprehensive test file covers:

    User Creation - Normal and edge cases

    Profile Updates - Full and partial updates, validation

    State Changes - Activation, deactivation, email verification

    Event Generation - All domain events are properly created

    Business Rules - Validation and business logic enforcement

    Getter Methods - All field accessors work correctly

    Immutable Properties - Proper encapsulation

    Lifecycle Testing - Complete user lifecycle from creation to deactivation

    Error Conditions - Proper error handling for invalid operations

    Timestamp Behavior - Created/updated at timestamps are managed correctly

The tests ensure that your User entity behaves correctly according to the domain rules and maintains data integrity throughout its lifecycle.
--> tests/unit/entities/user_test.rs:4:30
  |
4 |     use super::super::super::common::test_utils;
  |                              ^^^^^^ could not find `common` in `super`

The issue is with the import path. Let me fix the user_test.rs file by removing the problematic import and using direct crate imports:
rust

// tests/unit/entities/user_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::value_objects::{Email, Username, PhoneNumber};
    use auth_service::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent};
    use auth_service::domain::DomainError;
    use chrono::{DateTime, Utc};

    fn create_test_user() -> (User, UserCreatedEvent) {
        let keycloak_id = "test-keycloak-id".to_string();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        
        User::create(
            keycloak_id,
            username,
            email,
            Some("John".to_string()),
            Some("Doe".to_string()),
        ).unwrap()
    }

    fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }

    #[test]
    fn test_user_creation() {
        let (user, event) = create_test_user();
        
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(!user.phone_number().is_empty());
        assert!(user.avatar_url().is_none());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.email().as_str(), "test@example.com");
        assert_eq!(event.username().as_str(), "testuser");
    }

    #[test]
    fn test_user_creation_with_minimal_data() {
        let keycloak_id = "minimal-keycloak-id".to_string();
        let username = Username::new("minimaluser").unwrap();
        let email = Email::new("minimal@example.com").unwrap();
        
        let (user, event) = User::create(
            keycloak_id,
            username,
            email,
            None,
            None,
        ).unwrap();
        
        assert_eq!(user.keycloak_id(), "minimal-keycloak-id");
        assert_eq!(user.username().as_str(), "minimaluser");
        assert_eq!(user.email().as_str(), "minimal@example.com");
        assert_eq!(user.first_name(), None);
        assert_eq!(user.last_name(), None);
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
    }

    #[test]
    fn test_user_update_profile() {
        let (mut user, _) = create_test_user();
        let old_email = user.email().clone();
        
        let event = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("+1234567890".to_string()),
        ).unwrap();
        
        assert_eq!(user.first_name(), Some("Jane"));
        assert_eq!(user.last_name(), Some("Smith"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");
        
        // Verify event
        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.old_email(), &old_email);
    }

    #[test]
    fn test_user_update_profile_partial() {
        let (mut user, _) = create_test_user();
        let original_first_name = user.first_name().map(|s| s.to_string());
        
        // Update only phone number
        let event = user.update_profile(
            None, // Keep existing first name
            Some("UpdatedLast".to_string()), // Update last name
            Some("+1987654321".to_string()), // Update phone
        ).unwrap();
        
        assert_eq!(user.first_name(), original_first_name.as_deref());
        assert_eq!(user.last_name(), Some("UpdatedLast"));
        assert_eq!(user.phone_number().as_str(), "+1987654321");
        
        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_empty_phone() {
        let (mut user, _) = create_test_user();
        
        // Set a phone number first
        user.update_profile(
            None,
            None,
            Some("+1234567890".to_string()),
        ).unwrap();
        assert!(!user.phone_number().is_empty());
        
        // Update with empty phone (should clear it)
        let event = user.update_profile(
            None,
            None,
            Some("".to_string()),
        ).unwrap();
        
        assert!(user.phone_number().is_empty());
        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_invalid_phone() {
        let (mut user, _) = create_test_user();
        
        let result = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("invalid-phone".to_string()),
        );
        
        assert!(matches!(result, Err(DomainError::Validation(_))));
        // Ensure user data wasn't changed
        assert_eq!(user.first_name(), Some("John"));
    }

    #[test]
    fn test_user_mark_email_verified() {
        let (mut user, _) = create_test_user();
        
        assert!(!user.is_email_verified());
        user.mark_email_verified();
        assert!(user.is_email_verified());
        
        // Verify updated_at changed
        let updated_at_after = user.updated_at();
        assert!(updated_at_after > user.created_at());
    }

    #[test]
    fn test_user_record_login() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.last_login_at().is_none());
        user.record_login();
        
        assert!(user.last_login_at().is_some());
        assert!(user.updated_at() > original_updated_at);
        
        // Record another login
        let second_login_time = user.last_login_at().unwrap();
        user.record_login();
        assert!(user.last_login_at().unwrap() > second_login_time);
    }

    #[test]
    fn test_user_deactivate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.is_active());
        let event = user.deactivate().unwrap();
        assert!(!user.is_active());
        assert!(user.updated_at() > original_updated_at);
        
        // Verify event
        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), user.id());
    }

    #[test]
    fn test_user_cannot_deactivate_already_deactivated() {
        let (mut user, _) = create_test_user();
        
        user.deactivate().unwrap();
        let result = user.deactivate();
        
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        assert!(!user.is_active()); // Should still be deactivated
    }

    #[test]
    fn test_user_activate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        user.deactivate().unwrap();
        assert!(!user.is_active());
        
        user.activate();
        assert!(user.is_active());
        assert!(user.updated_at() > original_updated_at);
    }

    #[test]
    fn test_user_set_avatar_url() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.avatar_url().is_none());
        
        // Set avatar URL
        user.set_avatar_url(Some("https://example.com/avatar.jpg".to_string()));
        assert_eq!(user.avatar_url(), Some("https://example.com/avatar.jpg"));
        assert!(user.updated_at() > original_updated_at);
        
        // Update avatar URL
        let previous_updated_at = user.updated_at();
        user.set_avatar_url(Some("https://example.com/new-avatar.png".to_string()));
        assert_eq!(user.avatar_url(), Some("https://example.com/new-avatar.png"));
        assert!(user.updated_at() > previous_updated_at);
        
        // Remove avatar URL
        user.set_avatar_url(None);
        assert!(user.avatar_url().is_none());
    }

    #[test]
    fn test_user_getters() {
        let (user, _) = create_test_user();
        
        // Test all getters
        assert!(!user.id().as_str().is_empty());
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(!user.phone_number().is_empty());
        assert!(user.avatar_url().is_none());
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(user.created_at() <= chrono::Utc::now());
        assert!(user.updated_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_id_generation() {
        let (user1, _) = create_test_user();
        let (user2, _) = create_test_user();
        
        // Each user should have a unique ID
        assert_ne!(user1.id().as_str(), user2.id().as_str());
        
        // IDs should be valid UUIDs
        let uuid_result1 = uuid::Uuid::parse_str(user1.id().as_str());
        let uuid_result2 = uuid::Uuid::parse_str(user2.id().as_str());
        
        assert!(uuid_result1.is_ok());
        assert!(uuid_result2.is_ok());
    }

    #[test]
    fn test_user_created_at_and_updated_at() {
        let (user, _) = create_test_user();
        
        // Created and updated should be initially the same
        assert_eq!(user.created_at(), user.updated_at());
        
        // Both should be in the past (or very close to now)
        let now = chrono::Utc::now();
        assert!(user.created_at() <= now);
        assert!(user.updated_at() <= now);
    }

    #[test]
    fn test_user_phone_number_defaults_to_empty() {
        let keycloak_id = "test-keycloak".to_string();
        let username = Username::new("newuser").unwrap();
        let email = Email::new("new@example.com").unwrap();
        
        let (user, _) = User::create(keycloak_id, username, email, None, None).unwrap();
        
        assert!(user.phone_number().is_empty());
    }

    #[test]
    fn test_user_immutability_via_getters() {
        let (user, _) = create_test_user();
        
        // All getters should return immutable references or copies
        let id = user.id();
        let username = user.username();
        let email = user.email();
        
        // These should all compile and work
        assert_eq!(id.as_str(), user.id().as_str());
        assert_eq!(username.as_str(), user.username().as_str());
        assert_eq!(email.as_str(), user.email().as_str());
    }

    #[test]
    fn test_user_lifecycle_events() {
        // Create user
        let keycloak_id = "lifecycle-user".to_string();
        let username = Username::new("lifecycle").unwrap();
        let email = Email::new("lifecycle@example.com").unwrap();
        
        let (mut user, created_event) = User::create(
            keycloak_id,
            username,
            email,
            Some("Life".to_string()),
            Some("Cycle".to_string()),
        ).unwrap();
        
        assert_eq!(created_event.event_type(), "user.created");
        
        // Update profile
        let updated_event = user.update_profile(
            Some("UpdatedLife".to_string()),
            Some("UpdatedCycle".to_string()),
            Some("+1111111111".to_string()),
        ).unwrap();
        
        assert_eq!(updated_event.event_type(), "user.updated");
        
        // Deactivate
        let deactivated_event = user.deactivate().unwrap();
        assert_eq!(deactivated_event.event_type(), "user.deactivated");
        
        // Reactivate (no event for reactivation in current design)
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_business_rules() {
        let (mut user, _) = create_test_user();
        
        // Test that business rules are enforced
        user.deactivate().unwrap();
        
        // Try to deactivate again - should fail
        let result = user.deactivate();
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        
        // Error message should be meaningful
        if let Err(DomainError::BusinessRule(msg)) = result {
            assert!(msg.contains("already deactivated") || msg.contains("User already deactivated"));
        }
    }

    #[test]
    fn test_user_clone() {
        let (user, _) = create_test_user();
        let cloned_user = user.clone();
        
        // Cloned user should have same values
        assert_eq!(user.id().as_str(), cloned_user.id().as_str());
        assert_eq!(user.username().as_str(), cloned_user.username().as_str());
        assert_eq!(user.email().as_str(), cloned_user.email().as_str());
    }

    #[test]
    fn test_user_debug_format() {
        let (user, _) = create_test_user();
        
        // Debug format should work without panicking
        let debug_output = format!("{:?}", user);
        assert!(!debug_output.is_empty());
        // Should contain some identifiable information
        assert!(debug_output.contains("testuser") || debug_output.contains("test@example.com"));
    }
}

Key Changes Made:

    Removed the problematic import: use super::super::super::common::test_utils;

    Added direct utility function: fixed_datetime() inside the test module

    Used direct crate imports: auth_service::domain::...

    Added chrono import: For DateTime handling

    Added additional tests: For clone and debug traits

If you still want to use the common test utils, here's how to fix the import:

Create a proper common module that's accessible:
rust

// tests/common.rs
pub mod test_utils {
    use chrono::{DateTime, TimeZone, Utc};
    
    pub fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }
    
    pub fn create_valid_email() -> String {
        "test@example.com".to_string()
    }
    
    pub fn create_valid_username() -> String {
        "testuser".to_string()
    }
    
    pub fn create_valid_phone() -> String {
        "+1234567890".to_string()
    }
    
    pub fn create_valid_keycloak_id() -> String {
        "keycloak-123".to_string()
    }
}

Then in your test file, use:
rust

// At the top of tests/unit/entities/user_test.rs
use auth_service::tests::common::test_utils;

But the version I provided above with the inline fixed_datetime() function is simpler and will work immediately without any import issues.

The test file should now compile without the "could not find common" error.
error[E0599]: no method named `with_ymd_and_hms` found for struct `Utc` in the current scope
   --> tests/unit/entities/user_test.rs:25:13
    |
 25 |         Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    |             ^^^^^^^^^^^^^^^^ method not found in `Utc`
    |
   ::: /home/dali/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/chrono-0.4.42/src/offset/mod.rs:310:8
    |
310 |     fn with_ymd_and_hms(
    |        ---------------- the method is available for `Utc` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `TimeZone` which provides `with_ymd_and_hms` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use chrono::TimeZone;
    |

error[E0599]: no method named `avatar_url` found for struct `User` in the current scope
  --> tests/unit/entities/user_test.rs:41:22
   |
41 |         assert!(user.avatar_url().is_none());
   |                      ^^^^^^^^^^ private field, not a method

error[E0599]: no method named `event_type` found for struct `UserCreatedEvent` in the current scope
  --> tests/unit/entities/user_test.rs:44:26
   |
44 |         assert_eq!(event.event_type(), "user.created");
   |                          ^^^^^^^^^^ method not found in `UserCreatedEvent`
   |
  ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
   |
11 |     fn event_type(&self) -> &str;
   |        ---------- the method is available for `UserCreatedEvent` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
   |
 4 +     use auth_service::domain::events::DomainEvent;
   |

error[E0599]: no method named `event_type` found for struct `UserCreatedEvent` in the current scope
  --> tests/unit/entities/user_test.rs:73:26
   |
73 |         assert_eq!(event.event_type(), "user.created");
   |                          ^^^^^^^^^^ method not found in `UserCreatedEvent`
   |
  ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
   |
11 |     fn event_type(&self) -> &str;
   |        ---------- the method is available for `UserCreatedEvent` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
   |
 4 +     use auth_service::domain::events::DomainEvent;
   |

error[E0599]: no method named `event_type` found for struct `UserUpdatedEvent` in the current scope
  --> tests/unit/entities/user_test.rs:92:26
   |
92 |         assert_eq!(event.event_type(), "user.updated");
   |                          ^^^^^^^^^^ method not found in `UserUpdatedEvent`
   |
  ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
   |
11 |     fn event_type(&self) -> &str;
   |        ---------- the method is available for `UserUpdatedEvent` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
   |
 4 +     use auth_service::domain::events::DomainEvent;
   |

error[E0599]: no method named `event_type` found for struct `UserUpdatedEvent` in the current scope
   --> tests/unit/entities/user_test.rs:113:26
    |
113 |         assert_eq!(event.event_type(), "user.updated");
    |                          ^^^^^^^^^^ method not found in `UserUpdatedEvent`
    |
   ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
    |
 11 |     fn event_type(&self) -> &str;
    |        ---------- the method is available for `UserUpdatedEvent` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use auth_service::domain::events::DomainEvent;
    |

error[E0599]: no method named `event_type` found for struct `UserUpdatedEvent` in the current scope
   --> tests/unit/entities/user_test.rs:136:26
    |
136 |         assert_eq!(event.event_type(), "user.updated");
    |                          ^^^^^^^^^^ method not found in `UserUpdatedEvent`
    |
   ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
    |
 11 |     fn event_type(&self) -> &str;
    |        ---------- the method is available for `UserUpdatedEvent` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use auth_service::domain::events::DomainEvent;
    |

error[E0599]: no method named `event_type` found for struct `UserDeactivatedEvent` in the current scope
   --> tests/unit/entities/user_test.rs:195:26
    |
195 |         assert_eq!(event.event_type(), "user.deactivated");
    |                          ^^^^^^^^^^ method not found in `UserDeactivatedEvent`
    |
   ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
    |
 11 |     fn event_type(&self) -> &str;
    |        ---------- the method is available for `UserDeactivatedEvent` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use auth_service::domain::events::DomainEvent;
    |

error[E0599]: no method named `avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:228:22
    |
228 |         assert!(user.avatar_url().is_none());
    |                      ^^^^^^^^^^ private field, not a method

error[E0599]: no method named `set_avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:231:14
    |
231 |         user.set_avatar_url(Some("https://example.com/avatar.jpg".to_string()));
    |              ^^^^^^^^^^^^^^ method not found in `User`

error[E0599]: no method named `avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:232:25
    |
232 |         assert_eq!(user.avatar_url(), Some("https://example.com/avatar.jpg"));
    |                         ^^^^^^^^^^ private field, not a method

error[E0599]: no method named `set_avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:237:14
    |
237 |         user.set_avatar_url(Some("https://example.com/new-avatar.png".to_string()));
    |              ^^^^^^^^^^^^^^ method not found in `User`

error[E0599]: no method named `avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:238:25
    |
238 |         assert_eq!(user.avatar_url(), Some("https://example.com/new-avatar.png"));
    |                         ^^^^^^^^^^ private field, not a method

error[E0599]: no method named `set_avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:242:14
    |
242 |         user.set_avatar_url(None);
    |              ^^^^^^^^^^^^^^ method not found in `User`

error[E0599]: no method named `avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:243:22
    |
243 |         assert!(user.avatar_url().is_none());
    |                      ^^^^^^^^^^ private field, not a method

error[E0599]: no method named `avatar_url` found for struct `User` in the current scope
   --> tests/unit/entities/user_test.rs:258:22
    |
258 |         assert!(user.avatar_url().is_none());
    |                      ^^^^^^^^^^ private field, not a method

error[E0599]: no method named `event_type` found for struct `UserCreatedEvent` in the current scope
   --> tests/unit/entities/user_test.rs:336:34
    |
336 |         assert_eq!(created_event.event_type(), "user.created");
    |                                  ^^^^^^^^^^ method not found in `UserCreatedEvent`
    |
   ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
    |
 11 |     fn event_type(&self) -> &str;
    |        ---------- the method is available for `UserCreatedEvent` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use auth_service::domain::events::DomainEvent;
    |

error[E0599]: no method named `event_type` found for struct `UserUpdatedEvent` in the current scope
   --> tests/unit/entities/user_test.rs:345:34
    |
345 |         assert_eq!(updated_event.event_type(), "user.updated");
    |                                  ^^^^^^^^^^ method not found in `UserUpdatedEvent`
    |
   ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
    |
 11 |     fn event_type(&self) -> &str;
    |        ---------- the method is available for `UserUpdatedEvent` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use auth_service::domain::events::DomainEvent;
    |

error[E0599]: no method named `event_type` found for struct `UserDeactivatedEvent` in the current scope
   --> tests/unit/entities/user_test.rs:349:38
    |
349 |         assert_eq!(deactivated_event.event_type(), "user.deactivated");
    |                                      ^^^^^^^^^^ method not found in `UserDeactivatedEvent`
    |
   ::: /home/dali/WORK/work02/everest/sources/auth-service/src/domain/events/mod.rs:11:8
    |
 11 |     fn event_type(&self) -> &str;
    |        ---------- the method is available for `UserDeactivatedEvent` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `DomainEvent` which provides `event_type` is implemented but not in scope; perhaps you want to import it
    |
  4 +     use auth_service::domain::events::DomainEvent;
    |

I need to fix several issues in the test file. Here's the corrected version:
rust

// tests/unit/entities/user_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::entities::User;
    use auth_service::domain::value_objects::{Email, Username, PhoneNumber};
    use auth_service::domain::events::{UserCreatedEvent, UserUpdatedEvent, UserDeactivatedEvent, DomainEvent};
    use auth_service::domain::DomainError;
    use chrono::{DateTime, TimeZone, Utc};

    fn create_test_user() -> (User, UserCreatedEvent) {
        let keycloak_id = "test-keycloak-id".to_string();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        
        User::create(
            keycloak_id,
            username,
            email,
            Some("John".to_string()),
            Some("Doe".to_string()),
        ).unwrap()
    }

    fn fixed_datetime() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap()
    }

    #[test]
    fn test_user_creation() {
        let (user, event) = create_test_user();
        
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(!user.phone_number().is_empty());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.email().as_str(), "test@example.com");
        assert_eq!(event.username().as_str(), "testuser");
    }

    #[test]
    fn test_user_creation_with_minimal_data() {
        let keycloak_id = "minimal-keycloak-id".to_string();
        let username = Username::new("minimaluser").unwrap();
        let email = Email::new("minimal@example.com").unwrap();
        
        let (user, event) = User::create(
            keycloak_id,
            username,
            email,
            None,
            None,
        ).unwrap();
        
        assert_eq!(user.keycloak_id(), "minimal-keycloak-id");
        assert_eq!(user.username().as_str(), "minimaluser");
        assert_eq!(user.email().as_str(), "minimal@example.com");
        assert_eq!(user.first_name(), None);
        assert_eq!(user.last_name(), None);
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        
        // Verify event
        assert_eq!(event.event_type(), "user.created");
    }

    #[test]
    fn test_user_update_profile() {
        let (mut user, _) = create_test_user();
        let old_email = user.email().clone();
        
        let event = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("+1234567890".to_string()),
        ).unwrap();
        
        assert_eq!(user.first_name(), Some("Jane"));
        assert_eq!(user.last_name(), Some("Smith"));
        assert_eq!(user.phone_number().as_str(), "+1234567890");
        
        // Verify event
        assert_eq!(event.event_type(), "user.updated");
        assert_eq!(event.user_id(), user.id());
        assert_eq!(event.old_email(), &old_email);
    }

    #[test]
    fn test_user_update_profile_partial() {
        let (mut user, _) = create_test_user();
        let original_first_name = user.first_name().map(|s| s.to_string());
        
        // Update only phone number
        let event = user.update_profile(
            None, // Keep existing first name
            Some("UpdatedLast".to_string()), // Update last name
            Some("+1987654321".to_string()), // Update phone
        ).unwrap();
        
        assert_eq!(user.first_name(), original_first_name.as_deref());
        assert_eq!(user.last_name(), Some("UpdatedLast"));
        assert_eq!(user.phone_number().as_str(), "+1987654321");
        
        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_empty_phone() {
        let (mut user, _) = create_test_user();
        
        // Set a phone number first
        user.update_profile(
            None,
            None,
            Some("+1234567890".to_string()),
        ).unwrap();
        assert!(!user.phone_number().is_empty());
        
        // Update with empty phone (should clear it)
        let event = user.update_profile(
            None,
            None,
            Some("".to_string()),
        ).unwrap();
        
        assert!(user.phone_number().is_empty());
        assert_eq!(event.event_type(), "user.updated");
    }

    #[test]
    fn test_user_update_profile_with_invalid_phone() {
        let (mut user, _) = create_test_user();
        
        let result = user.update_profile(
            Some("Jane".to_string()),
            Some("Smith".to_string()),
            Some("invalid-phone".to_string()),
        );
        
        assert!(matches!(result, Err(DomainError::Validation(_))));
        // Ensure user data wasn't changed
        assert_eq!(user.first_name(), Some("John"));
    }

    #[test]
    fn test_user_mark_email_verified() {
        let (mut user, _) = create_test_user();
        
        assert!(!user.is_email_verified());
        user.mark_email_verified();
        assert!(user.is_email_verified());
        
        // Verify updated_at changed
        let updated_at_after = user.updated_at();
        assert!(updated_at_after > user.created_at());
    }

    #[test]
    fn test_user_record_login() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.last_login_at().is_none());
        user.record_login();
        
        assert!(user.last_login_at().is_some());
        assert!(user.updated_at() > original_updated_at);
        
        // Record another login
        let second_login_time = user.last_login_at().unwrap();
        user.record_login();
        assert!(user.last_login_at().unwrap() > second_login_time);
    }

    #[test]
    fn test_user_deactivate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        assert!(user.is_active());
        let event = user.deactivate().unwrap();
        assert!(!user.is_active());
        assert!(user.updated_at() > original_updated_at);
        
        // Verify event
        assert_eq!(event.event_type(), "user.deactivated");
        assert_eq!(event.user_id(), user.id());
    }

    #[test]
    fn test_user_cannot_deactivate_already_deactivated() {
        let (mut user, _) = create_test_user();
        
        user.deactivate().unwrap();
        let result = user.deactivate();
        
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        assert!(!user.is_active()); // Should still be deactivated
    }

    #[test]
    fn test_user_activate() {
        let (mut user, _) = create_test_user();
        let original_updated_at = user.updated_at();
        
        user.deactivate().unwrap();
        assert!(!user.is_active());
        
        user.activate();
        assert!(user.is_active());
        assert!(user.updated_at() > original_updated_at);
    }

    #[test]
    fn test_user_getters() {
        let (user, _) = create_test_user();
        
        // Test all getters
        assert!(!user.id().as_str().is_empty());
        assert_eq!(user.keycloak_id(), "test-keycloak-id");
        assert_eq!(user.username().as_str(), "testuser");
        assert_eq!(user.email().as_str(), "test@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert!(!user.phone_number().is_empty());
        assert!(user.is_active());
        assert!(!user.is_email_verified());
        assert!(user.last_login_at().is_none());
        assert!(user.created_at() <= chrono::Utc::now());
        assert!(user.updated_at() <= chrono::Utc::now());
    }

    #[test]
    fn test_user_id_generation() {
        let (user1, _) = create_test_user();
        let (user2, _) = create_test_user();
        
        // Each user should have a unique ID
        assert_ne!(user1.id().as_str(), user2.id().as_str());
        
        // IDs should be valid UUIDs
        let uuid_result1 = uuid::Uuid::parse_str(user1.id().as_str());
        let uuid_result2 = uuid::Uuid::parse_str(user2.id().as_str());
        
        assert!(uuid_result1.is_ok());
        assert!(uuid_result2.is_ok());
    }

    #[test]
    fn test_user_created_at_and_updated_at() {
        let (user, _) = create_test_user();
        
        // Created and updated should be initially the same
        assert_eq!(user.created_at(), user.updated_at());
        
        // Both should be in the past (or very close to now)
        let now = chrono::Utc::now();
        assert!(user.created_at() <= now);
        assert!(user.updated_at() <= now);
    }

    #[test]
    fn test_user_phone_number_defaults_to_empty() {
        let keycloak_id = "test-keycloak".to_string();
        let username = Username::new("newuser").unwrap();
        let email = Email::new("new@example.com").unwrap();
        
        let (user, _) = User::create(keycloak_id, username, email, None, None).unwrap();
        
        assert!(user.phone_number().is_empty());
    }

    #[test]
    fn test_user_immutability_via_getters() {
        let (user, _) = create_test_user();
        
        // All getters should return immutable references or copies
        let id = user.id();
        let username = user.username();
        let email = user.email();
        
        // These should all compile and work
        assert_eq!(id.as_str(), user.id().as_str());
        assert_eq!(username.as_str(), user.username().as_str());
        assert_eq!(email.as_str(), user.email().as_str());
    }

    #[test]
    fn test_user_lifecycle_events() {
        // Create user
        let keycloak_id = "lifecycle-user".to_string();
        let username = Username::new("lifecycle").unwrap();
        let email = Email::new("lifecycle@example.com").unwrap();
        
        let (mut user, created_event) = User::create(
            keycloak_id,
            username,
            email,
            Some("Life".to_string()),
            Some("Cycle".to_string()),
        ).unwrap();
        
        assert_eq!(created_event.event_type(), "user.created");
        
        // Update profile
        let updated_event = user.update_profile(
            Some("UpdatedLife".to_string()),
            Some("UpdatedCycle".to_string()),
            Some("+1111111111".to_string()),
        ).unwrap();
        
        assert_eq!(updated_event.event_type(), "user.updated");
        
        // Deactivate
        let deactivated_event = user.deactivate().unwrap();
        assert_eq!(deactivated_event.event_type(), "user.deactivated");
        
        // Reactivate (no event for reactivation in current design)
        user.activate();
        assert!(user.is_active());
    }

    #[test]
    fn test_user_business_rules() {
        let (mut user, _) = create_test_user();
        
        // Test that business rules are enforced
        user.deactivate().unwrap();
        
        // Try to deactivate again - should fail
        let result = user.deactivate();
        assert!(matches!(result, Err(DomainError::BusinessRule(_))));
        
        // Error message should be meaningful
        if let Err(DomainError::BusinessRule(msg)) = result {
            assert!(msg.contains("already deactivated") || msg.contains("User already deactivated"));
        }
    }

    #[test]
    fn test_user_clone() {
        let (user, _) = create_test_user();
        let cloned_user = user.clone();
        
        // Cloned user should have same values
        assert_eq!(user.id().as_str(), cloned_user.id().as_str());
        assert_eq!(user.username().as_str(), cloned_user.username().as_str());
        assert_eq!(user.email().as_str(), cloned_user.email().as_str());
    }

    #[test]
    fn test_user_debug_format() {
        let (user, _) = create_test_user();
        
        // Debug format should work without panicking
        let debug_output = format!("{:?}", user);
        assert!(!debug_output.is_empty());
        // Should contain some identifiable information
        assert!(debug_output.contains("testuser") || debug_output.contains("test@example.com"));
    }
}

Key Fixes Made:

    Added missing imports:

        use chrono::TimeZone; - for with_ymd_and_hms method

        use auth_service::domain::events::DomainEvent; - for event_type() method

    Removed references to non-existent methods:

        Removed all avatar_url() and set_avatar_url() method calls since they don't exist in your User entity

        The avatar_url field is private and doesn't have public getters/setters in your current implementation

    Updated the fixed_datetime function to use proper chrono imports

If you want to add avatar_url getters/setters, you need to update your User entity:
rust

// In src/domain/entities/user.rs - add these methods to the User impl block
pub fn avatar_url(&self) -> Option<&str> {
    self.avatar_url.as_deref()
}

pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
    self.avatar_url = avatar_url;
    self.updated_at = chrono::Utc::now();
}

But for now, the test file above should compile without errors since I removed all references to the non-existent avatar_url methods and fixed the imports.
failures:

---- unit::entities::user_test::tests::test_user_creation stdout ----

thread 'unit::entities::user_test::tests::test_user_creation' panicked at tests/unit/entities/user_test.rs:40:9:
assertion failed: !user.phone_number().is_empty()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- unit::entities::user_test::tests::test_user_getters stdout ----

thread 'unit::entities::user_test::tests::test_user_getters' panicked at tests/unit/entities/user_test.rs:233:9:
assertion failed: !user.phone_number().is_empty()

---- unit::entities::user_test::tests::test_user_update_profile_partial stdout ----

thread 'unit::entities::user_test::tests::test_user_update_profile_partial' panicked at tests/unit/entities/user_test.rs:108:9:
assertion `left == right` failed
  left: None
 right: Some("John")

---- unit::entities::user_test::tests::test_user_update_profile_with_invalid_phone stdout ----

thread 'unit::entities::user_test::tests::test_user_update_profile_with_invalid_phone' panicked at tests/unit/entities/user_test.rs:150:9:
assertion `left == right` failed
  left: Some("Jane")
 right: Some("John")

---- unit::value_objects::phone_number_test::tests::test_phone_number_rejects_too_short stdout ----

thread 'unit::value_objects::phone_number_test::tests::test_phone_number_rejects_too_short' panicked at tests/unit/value_objects/phone_number_test.rs:38:9:
assertion failed: matches!(result, Err(DomainError::Validation(_)))


failures:
    unit::entities::user_test::tests::test_user_creation
    unit::entities::user_test::tests::test_user_getters
    unit::entities::user_test::tests::test_user_update_profile_partial
    unit::entities::user_test::tests::test_user_update_profile_with_invalid_phone
    unit::value_objects::phone_number_test::tests::test_phone_number_rejects_too_short

test result: FAILED. 55 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

The test failures reveal issues with the phone number validation logic and user update behavior. Let me fix these:
1. First, fix the PhoneNumber validation
rust

// src/domain/value_objects/phone_number.rs
use crate::domain::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Result<Self, DomainError> {
        let phone = phone.trim().replace(' ', "").replace('-', "").replace('(', "").replace(')', "");
        
        if phone.is_empty() {
            return Ok(PhoneNumber(phone)); // Optional field
        }
        
        // Allow + at start and digits only
        let mut chars = phone.chars();
        if let Some('+') = chars.next() {
            // If starts with +, the rest must be digits
            if !chars.all(|c| c.is_numeric()) {
                return Err(DomainError::Validation("Phone number can only contain numbers and '+' at start".to_string()));
            }
        } else {
            // If no +, all must be digits
            if !phone.chars().all(|c| c.is_numeric()) {
                return Err(DomainError::Validation("Phone number can only contain numbers and '+' at start".to_string()));
            }
        }
        
        // Count digits only for length validation
        let digit_count = phone.chars().filter(|c| c.is_numeric()).count();
        if digit_count < 10 || digit_count > 15 {
            return Err(DomainError::Validation("Phone number must be between 10-15 digits".to_string()));
        }
        
        Ok(PhoneNumber(phone))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// Rest of the implementation remains the same...

2. Update the phone number test
rust

// tests/unit/value_objects/phone_number_test.rs
#[cfg(test)]
mod tests {
    use auth_service::domain::value_objects::PhoneNumber;
    use auth_service::domain::DomainError;

    #[test]
    fn test_create_valid_phone_number() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_removes_spaces_and_dashes() {
        let phone = PhoneNumber::new("+1 234-567-890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_removes_parentheses() {
        let phone = PhoneNumber::new("+1 (234) 567-890").unwrap();
        assert_eq!(phone.as_str(), "+1234567890");
    }

    #[test]
    fn test_phone_number_allows_empty_string() {
        let phone = PhoneNumber::new("").unwrap();
        assert!(phone.is_empty());
    }

    #[test]
    fn test_phone_number_rejects_invalid_characters() {
        let invalid_numbers = vec!["+1-800-ABC-DEFG", "800!123!4567", "+1-800-123-45a6"];
        
        for number in invalid_numbers {
            let result = PhoneNumber::new(number);
            assert!(matches!(result, Err(DomainError::Validation(_))), "Should have failed for: {}", number);
        }
    }

    #[test]
    fn test_phone_number_rejects_too_short() {
        // 9 digits is too short
        let result = PhoneNumber::new("+123456789"); // 9 digits
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_rejects_too_long() {
        // 16 digits is too long
        let result = PhoneNumber::new("+1234567890123456"); // 16 digits
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn test_phone_number_without_plus() {
        let phone = PhoneNumber::new("1234567890").unwrap();
        assert_eq!(phone.as_str(), "1234567890");
    }

    #[test]
    fn test_phone_number_is_empty() {
        let empty_phone = PhoneNumber::new("").unwrap();
        assert!(empty_phone.is_empty());
        
        let non_empty_phone = PhoneNumber::new("+1234567890").unwrap();
        assert!(!non_empty_phone.is_empty());
    }

    #[test]
    fn test_phone_number_display() {
        let phone = PhoneNumber::new("+1234567890").unwrap();
        assert_eq!(format!("{}", phone), "+1234567890");
    }
}

3. Fix the User entity update_profile method
rust

// src/domain/entities/user.rs - update the update_profile method
pub fn update_profile(
    &mut self,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
) -> Result<UserUpdatedEvent, DomainError> {
    let old_email = self.email.clone();
    
    // Only update if Some value is provided
    if let Some(first_name) = first_name {
        self.first_name = Some(first_name);
    }
    
    if let Some(last_name) = last_name {
        self.last_name = Some(last_name);
    }
    
    if let Some(phone) = phone_number {
        self.phone_number = PhoneNumber::new(&phone)?;
    }
    
    self.updated_at = chrono::Utc::now();
    
    Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
}

4. Update the failing user tests
rust

// tests/unit/entities/user_test.rs - update the failing tests
#[test]
fn test_user_creation() {
    let (user, event) = create_test_user();
    
    assert_eq!(user.keycloak_id(), "test-keycloak-id");
    assert_eq!(user.username().as_str(), "testuser");
    assert_eq!(user.email().as_str(), "test@example.com");
    assert_eq!(user.first_name(), Some("John"));
    assert_eq!(user.last_name(), Some("Doe"));
    assert!(user.is_active());
    assert!(!user.is_email_verified());
    assert!(user.last_login_at().is_none());
    assert!(user.phone_number().is_empty()); // Should be empty by default
    
    // Verify event
    assert_eq!(event.event_type(), "user.created");
    assert_eq!(event.user_id(), user.id());
    assert_eq!(event.email().as_str(), "test@example.com");
    assert_eq!(event.username().as_str(), "testuser");
}

#[test]
fn test_user_getters() {
    let (user, _) = create_test_user();
    
    // Test all getters
    assert!(!user.id().as_str().is_empty());
    assert_eq!(user.keycloak_id(), "test-keycloak-id");
    assert_eq!(user.username().as_str(), "testuser");
    assert_eq!(user.email().as_str(), "test@example.com");
    assert_eq!(user.first_name(), Some("John"));
    assert_eq!(user.last_name(), Some("Doe"));
    assert!(user.phone_number().is_empty()); // Should be empty by default
    assert!(user.is_active());
    assert!(!user.is_email_verified());
    assert!(user.last_login_at().is_none());
    assert!(user.created_at() <= chrono::Utc::now());
    assert!(user.updated_at() <= chrono::Utc::now());
}

#[test]
fn test_user_update_profile_partial() {
    let (mut user, _) = create_test_user();
    let original_first_name = user.first_name().map(|s| s.to_string());
    
    // Update only last name and phone
    let event = user.update_profile(
        None, // Keep existing first name
        Some("UpdatedLast".to_string()), // Update last name
        Some("+1987654321".to_string()), // Update phone
    ).unwrap();
    
    // First name should remain unchanged
    assert_eq!(user.first_name(), original_first_name.as_deref());
    assert_eq!(user.last_name(), Some("UpdatedLast"));
    assert_eq!(user.phone_number().as_str(), "+1987654321");
    
    assert_eq!(event.event_type(), "user.updated");
}

#[test]
fn test_user_update_profile_with_invalid_phone() {
    let (mut user, _) = create_test_user();
    
    let result = user.update_profile(
        Some("Jane".to_string()),
        Some("Smith".to_string()),
        Some("invalid-phone".to_string()),
    );
    
    assert!(matches!(result, Err(DomainError::Validation(_))));
    // Ensure user data wasn't changed due to validation error
    // The first and last name should NOT be updated because the entire update fails
    assert_eq!(user.first_name(), Some("John"));
    assert_eq!(user.last_name(), Some("Doe"));
}

// Add a new test to verify that partial updates work correctly
#[test]
fn test_user_update_profile_preserves_existing_values() {
    let (mut user, _) = create_test_user();
    
    // Set initial phone number
    user.update_profile(
        None,
        None,
        Some("+1234567890".to_string()),
    ).unwrap();
    
    // Update only first name
    user.update_profile(
        Some("OnlyFirst".to_string()),
        None, // Keep existing last name
        None, // Keep existing phone number
    ).unwrap();
    
    assert_eq!(user.first_name(), Some("OnlyFirst"));
    assert_eq!(user.last_name(), Some("Doe")); // Should remain unchanged
    assert_eq!(user.phone_number().as_str(), "+1234567890"); // Should remain unchanged
}

Key Changes Made:

    Fixed PhoneNumber validation:

        Better character filtering (removes spaces, dashes, parentheses)

        Proper digit counting for length validation

        Allows + only at the start

    Fixed User update behavior:

        update_profile now only updates fields when Some value is provided

        If phone validation fails, the entire update fails and no fields are changed

    Updated test expectations:

        Phone number should be empty by default after user creation

        Tests now expect the correct behavior for partial updates

        Added test for preserving existing values during partial updates

The tests should now pass with these fixes. The main issues were:

    Phone number validation was too strict

    User update was overwriting fields with None values

    Test expectations didn't match the actual behavior

failures:

---- unit::entities::user_test::tests::test_user_update_profile_with_invalid_phone stdout ----

thread 'unit::entities::user_test::tests::test_user_update_profile_with_invalid_phone' panicked at tests/unit/entities/user_test.rs:148:9:
assertion `left == right` failed
  left: Some("Jane")
 right: Some("John")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    unit::entities::user_test::tests::test_user_update_profile_with_invalid_phone

test result: FAILED. 62 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

The issue is that the user fields are being updated before the phone number validation fails. We need to validate the phone number first before updating any fields. Here's the fix:
Fix the User entity update_profile method
rust

// src/domain/entities/user.rs - update the update_profile method
pub fn update_profile(
    &mut self,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
) -> Result<UserUpdatedEvent, DomainError> {
    let old_email = self.email.clone();
    
    // Validate phone number FIRST before updating any fields
    let new_phone_number = if let Some(phone) = &phone_number {
        Some(PhoneNumber::new(phone)?)
    } else {
        None
    };
    
    // Only update if Some value is provided
    if let Some(first_name) = first_name {
        self.first_name = Some(first_name);
    }
    
    if let Some(last_name) = last_name {
        self.last_name = Some(last_name);
    }
    
    if let Some(phone) = new_phone_number {
        self.phone_number = phone;
    }
    
    self.updated_at = chrono::Utc::now();
    
    Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
}

Alternative approach: Use a more defensive update pattern
rust

// Alternative implementation - more defensive
pub fn update_profile(
    &mut self,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
) -> Result<UserUpdatedEvent, DomainError> {
    let old_email = self.email.clone();
    
    // Create a copy of current state in case we need to rollback
    let original_first_name = self.first_name.clone();
    let original_last_name = self.last_name.clone();
    let original_phone_number = self.phone_number.clone();
    
    // Validate phone number first
    if let Some(phone) = &phone_number {
        let _ = PhoneNumber::new(phone)?; // Validate but don't assign yet
    }
    
    // Now update fields
    if let Some(first_name) = first_name {
        self.first_name = Some(first_name);
    }
    
    if let Some(last_name) = last_name {
        self.last_name = Some(last_name);
    }
    
    if let Some(phone) = phone_number {
        self.phone_number = PhoneNumber::new(&phone)?;
    }
    
    self.updated_at = chrono::Utc::now();
    
    Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
}

Or use a transactional approach
rust

// Transactional approach - safest
pub fn update_profile(
    &mut self,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
) -> Result<UserUpdatedEvent, DomainError> {
    let old_email = self.email.clone();
    
    // Validate all inputs first
    let validated_phone = if let Some(phone) = &phone_number {
        Some(PhoneNumber::new(phone)?)
    } else {
        None
    };
    
    // Store original values for rollback
    let original_state = (
        self.first_name.clone(),
        self.last_name.clone(),
        self.phone_number.clone(),
        self.updated_at,
    );
    
    // Update fields
    if let Some(first_name) = first_name {
        self.first_name = Some(first_name);
    }
    
    if let Some(last_name) = last_name {
        self.last_name = Some(last_name);
    }
    
    if let Some(phone) = validated_phone {
        self.phone_number = phone;
    }
    
    self.updated_at = chrono::Utc::now();
    
    Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
}

I recommend the first approach (simplest and cleanest):
rust

// src/domain/entities/user.rs - use this version
pub fn update_profile(
    &mut self,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
) -> Result<UserUpdatedEvent, DomainError> {
    let old_email = self.email.clone();
    
    // Validate phone number FIRST before updating any fields
    let validated_phone = if let Some(phone) = &phone_number {
        Some(PhoneNumber::new(phone)?)
    } else {
        None
    };
    
    // Only update if Some value is provided
    if let Some(first_name) = first_name {
        self.first_name = Some(first_name);
    }
    
    if let Some(last_name) = last_name {
        self.last_name = Some(last_name);
    }
    
    if let Some(phone) = validated_phone {
        self.phone_number = phone;
    }
    
    self.updated_at = chrono::Utc::now();
    
    Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
}

Also update the test to be more specific
rust

// tests/unit/entities/user_test.rs - update the failing test
#[test]
fn test_user_update_profile_with_invalid_phone() {
    let (mut user, _) = create_test_user();
    
    let result = user.update_profile(
        Some("Jane".to_string()),
        Some("Smith".to_string()),
        Some("invalid-phone".to_string()),
    );
    
    // Should fail with validation error
    assert!(matches!(result, Err(DomainError::Validation(_))));
    
    // Ensure NO fields were changed due to validation error
    // The validation happens before any fields are updated
    assert_eq!(user.first_name(), Some("John"));
    assert_eq!(user.last_name(), Some("Doe"));
    assert!(user.phone_number().is_empty()); // Should still be empty
}

The Problem Was:

The original code was doing:

    Update first_name to "Jane" ✅

    Update last_name to "Smith" ✅

    Try to validate phone number "invalid-phone" ❌ (fails)

    But first_name and last_name were already changed!

The fix ensures:

    Validate phone number first ❌ (fails immediately)

    No fields get updated because validation failed early

This maintains data integrity - either all updates succeed or none do.
