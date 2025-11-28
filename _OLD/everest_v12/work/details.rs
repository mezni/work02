
# Auth Service - Complete File Structure

```
auth-service/
├── Cargo.toml
├── .env.example
├── config.toml.example
├── README.md
├── docker-compose.yml           # Keycloak + PostgreSQL for local dev
├── migrations/
│   ├── 20240101000001_create_users_table.sql
│   ├── 20240101000002_create_audit_logs_table.sql
│   └── 20240101000003_create_sessions_table.sql
├── src/
│   ├── main.rs                  # Minimal entry point
│   ├── lib.rs                   # Library root, exports modules
│   │
│   ├── config/
│   │   ├── mod.rs              # Configuration module
│   │   └── settings.rs         # Settings struct and loaders
│   │
│   ├── domain/                 # Pure domain logic (no dependencies)
│   │   ├── mod.rs
│   │   ├── entities/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs         # User aggregate root
│   │   │   ├── audit_log.rs    # AuditLog entity
│   │   │   └── session.rs      # Session entity
│   │   ├── value_objects/
│   │   │   ├── mod.rs
│   │   │   ├── email.rs        # Email value object
│   │   │   ├── user_role.rs    # UserRole enum
│   │   │   ├── partner_scope.rs
│   │   │   └── geo_location.rs
│   │   ├── events/
│   │   │   ├── mod.rs
│   │   │   ├── user_events.rs  # UserCreated, UserUpdated, etc.
│   │   │   └── audit_events.rs # LoginAttempted, LogoutPerformed, etc.
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   └── user_domain_service.rs  # Business rules validation
│   │   └── repositories/       # Repository traits (interfaces)
│   │       ├── mod.rs
│   │       ├── user_repository.rs
│   │       └── audit_repository.rs
│   │
│   ├── application/            # Use cases, DTOs, orchestration
│   │   ├── mod.rs
│   │   ├── commands/           # Write operations (CQRS)
│   │   │   ├── mod.rs
│   │   │   ├── register_user.rs
│   │   │   ├── create_admin_user.rs
│   │   │   ├── update_user.rs
│   │   │   ├── delete_user.rs
│   │   │   ├── link_operator_to_partner.rs
│   │   │   └── record_audit_event.rs
│   │   ├── queries/            # Read operations (CQRS)
│   │   │   ├── mod.rs
│   │   │   ├── get_user.rs
│   │   │   ├── list_users.rs
│   │   │   ├── get_audit_logs.rs
│   │   │   └── get_user_sessions.rs
│   │   ├── dtos/
│   │   │   ├── mod.rs
│   │   │   ├── user_dto.rs
│   │   │   ├── audit_log_dto.rs
│   │   │   ├── create_user_request.rs
│   │   │   └── update_user_request.rs
│   │   ├── mappers/
│   │   │   ├── mod.rs
│   │   │   ├── user_mapper.rs
│   │   │   └── audit_mapper.rs
│   │   └── errors/
│   │       ├── mod.rs
│   │       └── service_error.rs  # Unified error enum
│   │
│   ├── infrastructure/         # External integrations
│   │   ├── mod.rs
│   │   ├── keycloak/
│   │   │   ├── mod.rs
│   │   │   ├── client.rs       # Keycloak admin client wrapper
│   │   │   ├── user_service.rs # User CRUD via Keycloak
│   │   │   └── token_validator.rs  # JWT validation
│   │   ├── persistence/
│   │   │   ├── mod.rs
│   │   │   ├── postgres.rs     # Database connection pool
│   │   │   ├── user_repository_impl.rs
│   │   │   └── audit_repository_impl.rs
│   │   ├── logging/
│   │   │   ├── mod.rs
│   │   │   └── telemetry.rs    # Tracing setup
│   │   └── health/
│   │       ├── mod.rs
│   │       └── health_check.rs # Health check services
│   │
│   └── interfaces/             # API layer
│       ├── mod.rs
│       ├── http/
│       │   ├── mod.rs
│       │   ├── server.rs       # Actix-web server setup
│       │   ├── routes.rs       # Route configuration
│       │   ├── controllers/
│       │   │   ├── mod.rs
│       │   │   ├── user_controller.rs
│       │   │   ├── auth_controller.rs
│       │   │   ├── audit_controller.rs
│       │   │   └── health_controller.rs
│       │   ├── middleware/
│       │   │   ├── mod.rs
│       │   │   ├── authentication.rs  # JWT verification
│       │   │   ├── authorization.rs   # Role-based checks
│       │   │   ├── audit_logger.rs    # Request/response logging
│       │   │   └── error_handler.rs   # Global error handler
│       │   └── validators/
│       │       ├── mod.rs
│       │       └── request_validators.rs
│       └── openapi/
│           ├── mod.rs
│           └── spec.rs         # OpenAPI/Swagger configuration
│
└── tests/
    ├── integration/
    │   ├── mod.rs
    │   ├── user_api_tests.rs
    │   ├── auth_flow_tests.rs
    │   └── keycloak_integration_tests.rs
    └── common/
        ├── mod.rs
        └── test_helpers.rs     # Test utilities, fixtures
```

---

## Key Files Overview

### `Cargo.toml`
```toml
[package]
name = "auth-service"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
actix-web = "4"
actix-rt = "2"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "migrate", "uuid", "chrono"] }

# Keycloak
keycloak = "24"  # Or use reqwest directly
jsonwebtoken = "9"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Configuration
config = "0.14"
dotenv = "0.15"

# Logging & Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-actix-web = "0.7"

# OpenAPI/Swagger
utoipa = { version = "4", features = ["actix_extras"] }
utoipa-swagger-ui = { version = "6", features = ["actix-web"] }

# Error handling
thiserror = "1"
anyhow = "1"

# Validation
validator = { version = "0.16", features = ["derive"] }

# UUID
uuid = { version = "1", features = ["v4", "serde"] }

# DateTime
chrono = { version = "0.4", features = ["serde"] }

# Password hashing (if needed)
argon2 = "0.5"

[dev-dependencies]
testcontainers = "0.15"
```

---

## Database Migrations

### `migrations/20240101000001_create_users_table.sql`
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    role VARCHAR(50) NOT NULL,
    partner_name VARCHAR(255),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_partner_name ON users(partner_name);
```

### `migrations/20240101000002_create_audit_logs_table.sql`
```sql
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    event_type VARCHAR(100) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    device_type VARCHAR(50),
    os VARCHAR(50),
    browser VARCHAR(50),
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    success BOOLEAN,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
```

### `migrations/20240101000003_create_sessions_table.sql`
```sql
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    token_id VARCHAR(255) UNIQUE NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_token_id ON sessions(token_id);
```

---

## Configuration Files

### `.env.example`
```env
# Server
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# Database
DATABASE_URL=postgresql://postgres:password@localhost:5432/auth_service

# Keycloak
KEYCLOAK_URL=http://localhost:8081
KEYCLOAK_REALM=auth-realm
KEYCLOAK_CLIENT_ID=auth-service
KEYCLOAK_CLIENT_SECRET=your-client-secret

# Logging
LOG_LEVEL=info
RUST_LOG=info

# Security
JWT_SECRET=your-jwt-secret
```

### `config.toml.example`
```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgresql://postgres:password@localhost:5432/auth_service"
max_connections = 10

[keycloak]
url = "http://localhost:8081"
realm = "auth-realm"
client_id = "auth-service"
client_secret = "your-client-secret"

[logging]
level = "info"
```

---

## Docker Compose (Development)

### `docker-compose.yml`
```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: auth_service
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  keycloak:
    image: quay.io/keycloak/keycloak:23.0
    environment:
      KEYCLOAK_ADMIN: admin
      KEYCLOAK_ADMIN_PASSWORD: admin
      KC_DB: postgres
      KC_DB_URL: jdbc:postgresql://postgres:5432/keycloak
      KC_DB_USERNAME: postgres
      KC_DB_PASSWORD: password
    command: start-dev
    ports:
      - "8081:8080"
    depends_on:
      - postgres

volumes:
  postgres_data:
```

---

This structure follows DDD principles with clear separation of concerns, making the codebase maintainable and testable.

# Auth Service - API Endpoints

## Base URL
```
http://localhost:8080/api/v1
```

## OpenAPI/Swagger UI
```
http://localhost:8080/swagger-ui/
```

---

## Authentication Endpoints

### 1. Self-Registration
**POST** `/auth/register`

**Public Access** (No authentication required)

**Request Body:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe"
}
```

**Response:** `201 Created`
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "username": "johndoe",
  "role": "registered_user",
  "message": "Registration successful. Please check your email for activation."
}
```

**Errors:**
- `400 Bad Request` - Validation errors
- `409 Conflict` - Email or username already exists

---

### 2. Login
**POST** `/auth/login`

**Public Access**

**Request Body:**
```json
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Response:** `200 OK`
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "role": "registered_user"
  }
}
```

**Errors:**
- `401 Unauthorized` - Invalid credentials
- `403 Forbidden` - Account not activated

---

### 3. Logout
**POST** `/auth/logout`

**Requires:** Bearer token

**Response:** `200 OK`
```json
{
  "message": "Logout successful"
}
```

---

### 4. Refresh Token
**POST** `/auth/refresh`

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:** `200 OK`
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

---

## User Management Endpoints

### 5. Create User (Admin Only)
**POST** `/users`

**Requires:** `admin` role

**Request Body:**
```json
{
  "email": "operator@acme.com",
  "username": "acme_operator",
  "password": "SecurePass123!",
  "first_name": "Jane",
  "last_name": "Smith",
  "role": "operator",
  "partner_name": "AcmeCorp"
}
```

**Response:** `201 Created`
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440001",
  "keycloak_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "email": "operator@acme.com",
  "username": "acme_operator",
  "role": "operator",
  "partner_name": "AcmeCorp",
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z"
}
```

**Errors:**
- `400 Bad Request` - Validation errors
- `403 Forbidden` - Insufficient permissions
- `409 Conflict` - User already exists

---

### 6. Get User by ID
**GET** `/users/{user_id}`

**Requires:** Authentication

**Scoping Rules:**
- `admin`: Can view any user
- `partner`: Can view operators in their organization
- `operator`: Can view only their own profile
- `registered_user`: Can view only their own profile

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "keycloak_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "email": "user@example.com",
  "username": "johndoe",
  "first_name": "John",
  "last_name": "Doe",
  "role": "registered_user",
  "partner_name": null,
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

**Errors:**
- `403 Forbidden` - Cannot access this user
- `404 Not Found` - User not found

---

### 7. List Users
**GET** `/users`

**Requires:** `admin`, `partner`, or `operator` role

**Query Parameters:**
- `page` (default: 1)
- `limit` (default: 20, max: 100)
- `role` (filter by role)
- `partner_name` (filter by partner)
- `is_active` (filter by status)
- `search` (search by email/username)

**Scoping Rules:**
- `admin`: Can list all users
- `partner`: Can list only operators in their organization
- `operator`: Can list only themselves

**Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "user@example.com",
      "username": "johndoe",
      "role": "registered_user",
      "partner_name": null,
      "is_active": true,
      "created_at": "2024-01-01T12:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "total_pages": 8
  }
}
```

---

### 8. Update User
**PUT** `/users/{user_id}`

**Requires:** Authentication

**Scoping Rules:**
- `admin`: Can update any user
- `partner`: Can update operators in their organization
- User can update their own profile (limited fields)

**Request Body:**
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "email": "newemail@example.com"
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "newemail@example.com",
  "username": "johndoe",
  "first_name": "John",
  "last_name": "Doe",
  "role": "registered_user",
  "updated_at": "2024-01-02T10:30:00Z"
}
```

---

### 9. Delete User
**DELETE** `/users/{user_id}`

**Requires:** `admin` role

**Response:** `204 No Content`

**Errors:**
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - User not found

---

### 10. Activate/Deactivate User
**PATCH** `/users/{user_id}/status`

**Requires:** `admin` or `partner` role

**Request Body:**
```json
{
  "is_active": false
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "is_active": false,
  "updated_at": "2024-01-02T10:30:00Z"
}
```

---

### 11. Link Operator to Partner
**POST** `/users/{user_id}/link-partner`

**Requires:** `admin` role

**Request Body:**
```json
{
  "partner_name": "AcmeCorp"
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "operator123",
  "role": "operator",
  "partner_name": "AcmeCorp",
  "updated_at": "2024-01-02T10:30:00Z"
}
```

**Errors:**
- `400 Bad Request` - User must have operator role
- `404 Not Found` - Partner not found

---

## Audit Log Endpoints

### 12. Get User Audit Logs
**GET** `/audit/users/{user_id}`

**Requires:** `admin` or the user themselves

**Query Parameters:**
- `page` (default: 1)
- `limit` (default: 50, max: 100)
- `event_type` (filter by event type)
- `from_date` (ISO 8601 format)
- `to_date` (ISO 8601 format)

**Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "770e8400-e29b-41d4-a716-446655440003",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "event_type": "login",
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "device_type": "desktop",
      "os": "Windows 10",
      "browser": "Chrome 120",
      "latitude": 40.7128,
      "longitude": -74.0060,
      "success": true,
      "created_at": "2024-01-02T09:15:00Z"
    },
    {
      "id": "770e8400-e29b-41d4-a716-446655440004",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "event_type": "failed_login",
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "success": false,
      "error_message": "Invalid password",
      "created_at": "2024-01-02T09:10:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 245
  }
}
```

---

### 13. Get All Audit Logs (Admin)
**GET** `/audit`

**Requires:** `admin` role

**Query Parameters:**
- `page`, `limit`, `event_type`, `from_date`, `to_date` (same as above)
- `user_id` (filter by user)

**Response:** Same structure as endpoint #12

---

### 14. Get Audit Event Types
**GET** `/audit/event-types`

**Requires:** `admin` role

**Response:** `200 OK`
```json
{
  "event_types": [
    "login",
    "logout",
    "failed_login",
    "session_timeout",
    "user_created",
    "user_updated",
    "user_deleted",
    "password_changed",
    "role_changed"
  ]
}
```

---

## Session Management Endpoints

### 15. Get Active Sessions
**GET** `/sessions`

**Requires:** Authentication (user can view their own sessions, admin can view all)

**Response:** `200 OK`
```json
{
  "sessions": [
    {
      "id": "880e8400-e29b-41d4-a716-446655440005",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "expires_at": "2024-01-02T18:00:00Z",
      "created_at": "2024-01-02T09:00:00Z"
    }
  ]
}
```

---

### 16. Revoke Session
**DELETE** `/sessions/{session_id}`

**Requires:** Authentication (user can revoke their own sessions, admin can revoke any)

**Response:** `204 No Content`

---

## Health & Monitoring Endpoints

### 17. Health Check
**GET** `/health`

**Public Access**

**Response:** `200 OK`
```json
{
  "status": "healthy",
  "timestamp": "2024-01-02T10:00:00Z",
  "checks": {
    "database": {
      "status": "up",
      "response_time_ms": 5
    },
    "keycloak": {
      "status": "up",
      "response_time_ms": 12
    }
  }
}
```

**Errors:**
- `503 Service Unavailable` - One or more services are down
```json
{
  "status": "unhealthy",
  "timestamp": "2024-01-02T10:00:00Z",
  "checks": {
    "database": {
      "status": "up",
      "response_time_ms": 5
    },
    "keycloak": {
      "status": "down",
      "error": "Connection timeout"
    }
  }
}
```

---

### 18. Readiness Check
**GET** `/health/ready`

**Public Access**

**Response:** `200 OK` if service is ready to accept requests

---

### 19. Liveness Check
**GET** `/health/live`

**Public Access**

**Response:** `200 OK` if service is running

---

## Testing Checklist

### Authentication Flow
- [ ] Self-registration creates user in Keycloak with `registered_user` role
- [ ] Activation email is sent (check Keycloak events)
- [ ] Login returns valid JWT tokens
- [ ] Logout invalidates session
- [ ] Refresh token generates new access token

### Admin User Management
- [ ] Admin can create users with all roles
- [ ] Admin can assign operator to partner
- [ ] Admin can activate/deactivate users
- [ ] Admin can delete users

### Role-Based Access Control
- [ ] Public can only access registration and login
- [ ] Registered users can view/update their own profile
- [ ] Operators can only view their own data
- [ ] Partners can view/manage operators in their organization
- [ ] Admin has full system access

### Partner Scoping
- [ ] Operators are linked to exactly one partner
- [ ] Partner can only see operators with their `partner_name`
- [ ] Operators cannot access other operators' data

### Audit Logging
- [ ] Login events are recorded
- [ ] Failed login attempts are recorded
- [ ] Logout events are recorded
- [ ] IP address, user agent, and geolocation are captured
- [ ] Admin can view all audit logs
- [ ] Users can view their own audit logs

### Health Checks
- [ ] `/health` returns database status
- [ ] `/health` returns Keycloak status
- [ ] Service returns 503 when dependencies are unavailable

### Error Handling
- [ ] 400 for validation errors
- [ ] 401 for authentication failures
- [ ] 403 for authorization failures
- [ ] 404 for not found resources
- [ ] 409 for conflicts
- [ ] 500 for internal errors (with proper logging)

### Performance
- [ ] Response time < 200ms for simple queries
- [ ] Can handle 5 req/sec sustained load
- [ ] Can handle 50 concurrent users

---

## Example cURL Commands

### Self-Registration
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "SecurePass123!",
    "first_name": "Test",
    "last_name": "User"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "SecurePass123!"
  }'
```

### Get User Profile (Authenticated)
```bash
curl -X GET http://localhost:8080/api/v1/users/{user_id} \
  -H "Authorization: Bearer {access_token}"
```

### Health Check
```bash
curl -X GET http://localhost:8080/api/v1/health
```
# Auth Service - API Endpoints

## Base URL
```
http://localhost:8080/api/v1
```

## OpenAPI/Swagger UI
```
http://localhost:8080/swagger-ui/
```

---

## Authentication Endpoints

### 1. Self-Registration
**POST** `/auth/register`

**Public Access** (No authentication required)

**Request Body:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe"
}
```

**Response:** `201 Created`
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "username": "johndoe",
  "role": "registered_user",
  "message": "Registration successful. Please check your email for activation."
}
```

**Errors:**
- `400 Bad Request` - Validation errors
- `409 Conflict` - Email or username already exists

---

### 2. Login
**POST** `/auth/login`

**Public Access**

**Request Body:**
```json
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Response:** `200 OK`
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "role": "registered_user"
  }
}
```

**Errors:**
- `401 Unauthorized` - Invalid credentials
- `403 Forbidden` - Account not activated

---

### 3. Logout
**POST** `/auth/logout`

**Requires:** Bearer token

**Response:** `200 OK`
```json
{
  "message": "Logout successful"
}
```

---

### 4. Refresh Token
**POST** `/auth/refresh`

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:** `200 OK`
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

---

## User Management Endpoints

### 5. Create User (Admin Only)
**POST** `/users`

**Requires:** `admin` role

**Request Body:**
```json
{
  "email": "operator@acme.com",
  "username": "acme_operator",
  "password": "SecurePass123!",
  "first_name": "Jane",
  "last_name": "Smith",
  "role": "operator",
  "partner_name": "AcmeCorp"
}
```

**Response:** `201 Created`
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440001",
  "keycloak_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "email": "operator@acme.com",
  "username": "acme_operator",
  "role": "operator",
  "partner_name": "AcmeCorp",
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z"
}
```

**Errors:**
- `400 Bad Request` - Validation errors
- `403 Forbidden` - Insufficient permissions
- `409 Conflict` - User already exists

---

### 6. Get User by ID
**GET** `/users/{user_id}`

**Requires:** Authentication

**Scoping Rules:**
- `admin`: Can view any user
- `partner`: Can view operators in their organization
- `operator`: Can view only their own profile
- `registered_user`: Can view only their own profile

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "keycloak_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "email": "user@example.com",
  "username": "johndoe",
  "first_name": "John",
  "last_name": "Doe",
  "role": "registered_user",
  "partner_name": null,
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

**Errors:**
- `403 Forbidden` - Cannot access this user
- `404 Not Found` - User not found

---

### 7. List Users
**GET** `/users`

**Requires:** `admin`, `partner`, or `operator` role

**Query Parameters:**
- `page` (default: 1)
- `limit` (default: 20, max: 100)
- `role` (filter by role)
- `partner_name` (filter by partner)
- `is_active` (filter by status)
- `search` (search by email/username)

**Scoping Rules:**
- `admin`: Can list all users
- `partner`: Can list only operators in their organization
- `operator`: Can list only themselves

**Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "user@example.com",
      "username": "johndoe",
      "role": "registered_user",
      "partner_name": null,
      "is_active": true,
      "created_at": "2024-01-01T12:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "total_pages": 8
  }
}
```

---

### 8. Update User
**PUT** `/users/{user_id}`

**Requires:** Authentication

**Scoping Rules:**
- `admin`: Can update any user
- `partner`: Can update operators in their organization
- User can update their own profile (limited fields)

**Request Body:**
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "email": "newemail@example.com"
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "newemail@example.com",
  "username": "johndoe",
  "first_name": "John",
  "last_name": "Doe",
  "role": "registered_user",
  "updated_at": "2024-01-02T10:30:00Z"
}
```

---

### 9. Delete User
**DELETE** `/users/{user_id}`

**Requires:** `admin` role

**Response:** `204 No Content`

**Errors:**
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - User not found

---

### 10. Activate/Deactivate User
**PATCH** `/users/{user_id}/status`

**Requires:** `admin` or `partner` role

**Request Body:**
```json
{
  "is_active": false
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "is_active": false,
  "updated_at": "2024-01-02T10:30:00Z"
}
```

---

### 11. Link Operator to Partner
**POST** `/users/{user_id}/link-partner`

**Requires:** `admin` role

**Request Body:**
```json
{
  "partner_name": "AcmeCorp"
}
```

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "operator123",
  "role": "operator",
  "partner_name": "AcmeCorp",
  "updated_at": "2024-01-02T10:30:00Z"
}
```

**Errors:**
- `400 Bad Request` - User must have operator role
- `404 Not Found` - Partner not found

---

## Audit Log Endpoints

### 12. Get User Audit Logs
**GET** `/audit/users/{user_id}`

**Requires:** `admin` or the user themselves

**Query Parameters:**
- `page` (default: 1)
- `limit` (default: 50, max: 100)
- `event_type` (filter by event type)
- `from_date` (ISO 8601 format)
- `to_date` (ISO 8601 format)

**Response:** `200 OK`
```json
{
  "data": [
    {
      "id": "770e8400-e29b-41d4-a716-446655440003",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "event_type": "login",
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "device_type": "desktop",
      "os": "Windows 10",
      "browser": "Chrome 120",
      "latitude": 40.7128,
      "longitude": -74.0060,
      "success": true,
      "created_at": "2024-01-02T09:15:00Z"
    },
    {
      "id": "770e8400-e29b-41d4-a716-446655440004",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "event_type": "failed_login",
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "success": false,
      "error_message": "Invalid password",
      "created_at": "2024-01-02T09:10:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 245
  }
}
```

---

### 13. Get All Audit Logs (Admin)
**GET** `/audit`

**Requires:** `admin` role

**Query Parameters:**
- `page`, `limit`, `event_type`, `from_date`, `to_date` (same as above)
- `user_id` (filter by user)

**Response:** Same structure as endpoint #12

---

### 14. Get Audit Event Types
**GET** `/audit/event-types`

**Requires:** `admin` role

**Response:** `200 OK`
```json
{
  "event_types": [
    "login",
    "logout",
    "failed_login",
    "session_timeout",
    "user_created",
    "user_updated",
    "user_deleted",
    "password_changed",
    "role_changed"
  ]
}
```

---

## Session Management Endpoints

### 15. Get Active Sessions
**GET** `/sessions`

**Requires:** Authentication (user can view their own sessions, admin can view all)

**Response:** `200 OK`
```json
{
  "sessions": [
    {
      "id": "880e8400-e29b-41d4-a716-446655440005",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0...",
      "expires_at": "2024-01-02T18:00:00Z",
      "created_at": "2024-01-02T09:00:00Z"
    }
  ]
}
```

---

### 16. Revoke Session
**DELETE** `/sessions/{session_id}`

**Requires:** Authentication (user can revoke their own sessions, admin can revoke any)

**Response:** `204 No Content`

---

## Health & Monitoring Endpoints

### 17. Health Check
**GET** `/health`

**Public Access**

**Response:** `200 OK`
```json
{
  "status": "healthy",
  "timestamp": "2024-01-02T10:00:00Z",
  "checks": {
    "database": {
      "status": "up",
      "response_time_ms": 5
    },
    "keycloak": {
      "status": "up",
      "response_time_ms": 12
    }
  }
}
```

**Errors:**
- `503 Service Unavailable` - One or more services are down
```json
{
  "status": "unhealthy",
  "timestamp": "2024-01-02T10:00:00Z",
  "checks": {
    "database": {
      "status": "up",
      "response_time_ms": 5
    },
    "keycloak": {
      "status": "down",
      "error": "Connection timeout"
    }
  }
}
```

---

### 18. Readiness Check
**GET** `/health/ready`

**Public Access**

**Response:** `200 OK` if service is ready to accept requests

---

### 19. Liveness Check
**GET** `/health/live`

**Public Access**

**Response:** `200 OK` if service is running

---

## Testing Checklist

### Authentication Flow
- [ ] Self-registration creates user in Keycloak with `registered_user` role
- [ ] Activation email is sent (check Keycloak events)
- [ ] Login returns valid JWT tokens
- [ ] Logout invalidates session
- [ ] Refresh token generates new access token

### Admin User Management
- [ ] Admin can create users with all roles
- [ ] Admin can assign operator to partner
- [ ] Admin can activate/deactivate users
- [ ] Admin can delete users

### Role-Based Access Control
- [ ] Public can only access registration and login
- [ ] Registered users can view/update their own profile
- [ ] Operators can only view their own data
- [ ] Partners can view/manage operators in their organization
- [ ] Admin has full system access

### Partner Scoping
- [ ] Operators are linked to exactly one partner
- [ ] Partner can only see operators with their `partner_name`
- [ ] Operators cannot access other operators' data

### Audit Logging
- [ ] Login events are recorded
- [ ] Failed login attempts are recorded
- [ ] Logout events are recorded
- [ ] IP address, user agent, and geolocation are captured
- [ ] Admin can view all audit logs
- [ ] Users can view their own audit logs

### Health Checks
- [ ] `/health` returns database status
- [ ] `/health` returns Keycloak status
- [ ] Service returns 503 when dependencies are unavailable

### Error Handling
- [ ] 400 for validation errors
- [ ] 401 for authentication failures
- [ ] 403 for authorization failures
- [ ] 404 for not found resources
- [ ] 409 for conflicts
- [ ] 500 for internal errors (with proper logging)

### Performance
- [ ] Response time < 200ms for simple queries
- [ ] Can handle 5 req/sec sustained load
- [ ] Can handle 50 concurrent users

---

## Example cURL Commands

### Self-Registration
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "SecurePass123!",
    "first_name": "Test",
    "last_name": "User"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "SecurePass123!"
  }'
```

### Get User Profile (Authenticated)
```bash
curl -X GET http://localhost:8080/api/v1/users/{user_id} \
  -H "Authorization: Bearer {access_token}"
```

### Health Check
```bash
curl -X GET http://localhost:8080/api/v1/health
```