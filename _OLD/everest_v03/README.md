# EV Charging Auth Service - DDD Microservice

A Domain-Driven Design (DDD) authentication microservice built with Rust, Actix-web, PostgreSQL, and Keycloak for managing user authentication and authorization with role-based access control.

## Features

- ✅ **Two User Types**:
  - **Self-Registered** (role=USER, source=web) via `/api/v1/register`
  - **Admin-Created** (role=ADMIN/PARTNER/OPERATOR, source=internal) via `/api/v1/users`
- ✅ **Custom User IDs** - "USR" prefix + nanoid (e.g., `USRABC123XYZ456DEF789`)
- ✅ **Role-Based Access** - USER, ADMIN, PARTNER, OPERATOR
- ✅ **Keycloak Integration** - Full OAuth2/OIDC with roles and attributes
- ✅ **Network & Station Assignment** - For internal users (PARTNER, OPERATOR)
- ✅ **Password Management** - Users can change their own passwords
- ✅ **JWT Authentication** - Secure token-based auth
- ✅ **User Management** - CRUD operations with audit trail
- ✅ **Swagger/OpenAPI** - Complete API documentation
- ✅ **Health Check** - Service monitoring endpoint

## User Types

### 1. Self-Registered Users
```json
{
  "role": "USER",
  "source": "web",
  "network_id": "",
  "station_id": ""
}
```
- Register via `/api/v1/register`
- Automatically assigned USER role
- No network or station association
- Can: login, change password, update profile

### 2. Admin-Created Users
```json
{
  "role": "ADMIN | PARTNER | OPERATOR",
  "source": "internal",
  "network_id": "NET123...",
  "station_id": "STA456..."
}
```
- Created by admins via `/api/v1/users`
- Can have ADMIN, PARTNER, or OPERATOR roles
- Assigned to specific networks/stations
- Roles synced to Keycloak with attributes

## API Endpoints

### Public Endpoints

**Self-Register** (role=USER, source=web)
```bash
POST /api/v1/register
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1234567890"
}
```

**Login** (all roles)
```bash
POST /api/v1/login
{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

**Health Check**
```bash
GET /api/v1/health
```

### Protected Endpoints

**Create User** (Admin only, source=internal)
```bash
POST /api/v1/users
Authorization: Bearer {admin_token}
{
  "username": "operator1",
  "email": "operator@company.com",
  "password": "SecurePass123!",
  "role": "OPERATOR",
  "network_id": "NET123ABC",
  "station_id": "STA456DEF",
  "first_name": "Jane",
  "last_name": "Smith"
}
```

**Change Password**
```bash
POST /api/v1/users/{user_id}/change-password
Authorization: Bearer {token}
{
  "old_password": "OldPass123!",
  "new_password": "NewSecurePass456!"
}
```

**List Users** (Admin only)
```bash
GET /api/v1/users?role=OPERATOR&is_active=true
Authorization: Bearer {admin_token}
```

**Get User**
```bash
GET /api/v1/users/{user_id}
Authorization: Bearer {token}
```

**Update User**
```bash
PUT /api/v1/users/{user_id}
Authorization: Bearer {token}
{
  "first_name": "Jane",
  "last_name": "Smith",
  "phone": "+0987654321",
  "photo": "https://..."
}
```

**Deactivate User** (Admin only)
```bash
DELETE /api/v1/users/{user_id}
Authorization: Bearer {admin_token}
```

**Refresh Token**
```bash
POST /api/v1/auth/refresh
{
  "refresh_token": "{refresh_token}"
}
```

## Quick Start

### 1. Start Infrastructure

```bash
# Start Keycloak and PostgreSQL
docker-compose up -d
```

### 2. Configure Keycloak

1. Access: `http://localhost:8080` (admin/admin)
2. Create realm: `ev-charging`
3. Create client: `auth-service` (confidential)
4. Create roles: `USER`, `ADMIN`, `PARTNER`, `OPERATOR`
5. Copy client secret to `.env`

### 3. Setup Database

```bash
# Create database
createdb ev_charging

# Run migrations
psql -d ev_charging -f schema.sql
```

### 4. Configure Environment

```bash
cp .env.example .env
# Edit .env with your Keycloak client secret
```

### 5. Run Service

```bash
cargo run --release
```

Service starts at: `http://localhost:8081`
Swagger UI: `http://localhost:8081/swagger-ui/`

## Example Usage

### 1. Self-Register as User

```bash
curl -X POST http://localhost:8081/api/v1/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "user1",
    "email": "user1@example.com",
    "password": "Password123!",
    "first_name": "Regular",
    "last_name": "User"
  }'
```

Response:
```json
{
  "user_id": "USRABC123XYZ456789012",
  "message": "User registered successfully..."
}
```

### 2. Login

```bash
curl -X POST http://localhost:8081/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "user1",
    "password": "Password123!"
  }'
```

Response:
```json
{
  "access_token": "eyJhbGc...",
  "token_type": "Bearer",
  "expires_in": 300,
  "refresh_token": "eyJhbGc...",
  "refresh_expires_in": 1800
}
```

### 3. Admin Creates Operator

```bash
TOKEN="your-admin-token"

curl -X POST http://localhost:8081/api/v1/users \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "operator1",
    "email": "operator@company.com",
    "password": "SecureOp123!",
    "role": "OPERATOR",
    "network_id": "NET123ABC",
    "station_id": "STA456DEF",
    "first_name": "Station",
    "last_name": "Operator"
  }'
```

### 4. Change Password

```bash
curl -X POST http://localhost:8081/api/v1/users/USRABC123/change-password \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "old_password": "Password123!",
    "new_password": "NewPassword456!"
  }'
```

### 5. List All Operators

```bash
curl http://localhost:8081/api/v1/users?role=OPERATOR \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

## Database Schema

```sql
user_id VARCHAR(32)           -- USR + 21-char nanoid
keycloak_id VARCHAR(255)      -- Keycloak user ID
email VARCHAR(255)            -- Unique email
username VARCHAR(100)         -- Unique username
role VARCHAR(50)              -- USER, ADMIN, PARTNER, OPERATOR
network_id VARCHAR(32)        -- For PARTNER/OPERATOR
station_id VARCHAR(32)        -- For OPERATOR
source VARCHAR(20)            -- 'web' or 'internal'
is_verified BOOLEAN           -- Email verification status
photo TEXT                    -- Profile photo URL
created_by VARCHAR(32)        -- Audit: who created
updated_by VARCHAR(32)        -- Audit: who updated
```

## Keycloak Roles & Attributes

When creating users in Keycloak:
- **Role**: Assigned as realm role (USER, ADMIN, PARTNER, OPERATOR)
- **Attributes**: 
  - `network_id`: Network assignment
  - `station_id`: Station assignment

## Security

- ✅ Passwords stored only in Keycloak
- ✅ JWT tokens signed by Keycloak (RS256)
- ✅ Role-based access control
- ✅ Audit trail (created_by, updated_by)
- ✅ Password change requires old password verification
- ✅ All sensitive endpoints require authentication

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | PostgreSQL connection | Yes |
| `SERVER_HOST` | Server bind address | No (default: 0.0.0.0) |
| `SERVER_PORT` | Server port | No (default: 8081) |
| `KEYCLOAK_URL` | Keycloak server URL | Yes |
| `KEYCLOAK_REALM` | Keycloak realm name | Yes |
| `KEYCLOAK_CLIENT_ID` | Client ID | Yes |
| `KEYCLOAK_CLIENT_SECRET` | Client secret | Yes |
| `KEYCLOAK_ADMIN_USERNAME` | Admin username | Yes |
| `KEYCLOAK_ADMIN_PASSWORD` | Admin password | Yes |

## Technology Stack

- **Rust** - Systems programming language
- **Actix-web** - Web framework
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Database
- **Keycloak** - Identity and access management
- **Nanoid** - Unique ID generation
- **Utoipa** - OpenAPI/Swagger
- **Validator** - Input validation

## License

MIT

This service follows DDD principles with clear separation of concerns:

```
src/
├── domain/              # Domain entities and repository traits
│   ├── entities.rs      # User, TokenResponse, KeycloakUser
│   └── repositories.rs  # Repository trait definitions
├── application/         # Use cases and business logic
│   ├── dto.rs          # Data Transfer Objects
│   └── auth_service.rs # Authentication business logic
├── infrastructure/      # External concerns (DB, Keycloak, error handling)
│   ├── error.rs
│   ├── user_repository.rs
│   └── keycloak_client.rs
├── presentation/        # HTTP handlers and routes
│   ├── handlers.rs
│   ├── routes.rs
│   └── api_doc.rs      # OpenAPI/Swagger documentation
├── config.rs           # Configuration management
└── main.rs             # Application entry point
```

## Features

- ✅ **DDD Architecture** - Clean separation between domain, application, infrastructure, and presentation layers
- ✅ **Keycloak Integration** - Full integration with Keycloak for authentication and user management
- ✅ **User Registration** - Register new users with validation
- ✅ **User Login** - Authenticate users and issue JWT tokens
- ✅ **Token Refresh** - Refresh access tokens using refresh tokens
- ✅ **User Management** - CRUD operations for users
- ✅ **REST API** with `/api/v1` prefix
- ✅ **Swagger UI** documentation at `/swagger-ui/`
- ✅ **Health Check** endpoint
- ✅ **Error Handling** - Using thiserror and anyhow
- ✅ **Environment Configuration** - Using dotenvy

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- PostgreSQL 14+
- Keycloak 23+ (running instance)
- sqlx-cli: `cargo install sqlx-cli --no-default-features --features postgres`

## Keycloak Setup

### 1. Start Keycloak

Using Docker:
```bash
docker run -d \
  --name keycloak \
  -p 8080:8080 \
  -e KEYCLOAK_ADMIN=admin \
  -e KEYCLOAK_ADMIN_PASSWORD=admin \
  quay.io/keycloak/keycloak:23.0 \
  start-dev
```

### 2. Configure Keycloak

1. Access Keycloak admin console: `http://localhost:8080`
2. Login with `admin/admin`
3. Create a new realm called `ev-charging`
4. Create a client:
   - Client ID: `auth-service`
   - Client Protocol: `openid-connect`
   - Access Type: `confidential`
   - Valid Redirect URIs: `*`
   - Web Origins: `*`
5. Go to the **Credentials** tab and copy the **Secret**
6. Update your `.env` file with this secret

## Database Setup

### 1. Create Database

```bash
createdb ev_charging
```

### 2. Run Migrations

```bash
psql -d ev_charging -f schema.sql
```

## Configuration

### 1. Setup Environment

```bash
cp .env.example .env
```

### 2. Edit `.env`

```bash
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/ev_charging

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8081
RUST_LOG=info

# Keycloak
KEYCLOAK_URL=http://localhost:8080
KEYCLOAK_REALM=ev-charging
KEYCLOAK_CLIENT_ID=auth-service
KEYCLOAK_CLIENT_SECRET=your-client-secret-from-keycloak
KEYCLOAK_ADMIN_USERNAME=admin
KEYCLOAK_ADMIN_PASSWORD=admin
```

## Build and Run

```bash
# Build
cargo build --release

# Run
cargo run --release
```

The service will start at `http://localhost:8081`

## API Endpoints

### Health Check
```
GET /api/v1/health
```

### Authentication

**Register a new user**
```bash
POST /api/v1/auth/register
Content-Type: application/json

{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1234567890"
}
```

**Login**
```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "johndoe",
  "password": "SecurePass123!"
}
```

Response:
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 300,
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_expires_in": 1800
}
```

**Refresh Token**
```bash
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "your-refresh-token"
}
```

### User Management

**Get user by ID**
```bash
GET /api/v1/users/{user_id}
```

**Update user**
```bash
PUT /api/v1/users/{user_id}
Content-Type: application/json

{
  "first_name": "Jane",
  "last_name": "Smith",
  "phone": "+0987654321"
}
```

**Deactivate user**
```bash
DELETE /api/v1/users/{user_id}
```

## Example Usage with cURL

### Register a User
```bash
curl -X POST http://localhost:8081/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "SecurePass123!",
    "first_name": "Test",
    "last_name": "User"
  }'
```

### Login
```bash
curl -X POST http://localhost:8081/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "SecurePass123!"
  }'
```

### Get User (with authentication)
```bash
curl -X GET http://localhost:8081/api/v1/users/{user-id} \
  -H "Authorization: Bearer {access-token}"
```

## Swagger Documentation

Access the interactive API documentation at:
```
http://localhost:8081/swagger-ui/
```

## How It Works

### Registration Flow
1. Client sends registration request
2. Service validates input
3. Service checks if user exists in local database
4. Service creates user in Keycloak
5. Keycloak returns user ID
6. Service stores user in local database with Keycloak ID
7. Service returns success response

### Login Flow
1. Client sends username and password
2. Service forwards credentials to Keycloak
3. Keycloak validates and returns JWT tokens
4. Service returns tokens to client

### Token Refresh Flow
1. Client sends refresh token
2. Service forwards to Keycloak
3. Keycloak validates and issues new access token
4. Service returns new tokens

## Security Considerations

- Passwords are NEVER stored in the local database
- All password handling is done by Keycloak
- Tokens are signed and verified by Keycloak
- Use HTTPS in production
- Rotate Keycloak client secrets regularly
- Use strong passwords for Keycloak admin account
- Enable email verification in Keycloak for production

## Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Yes | - |
| `SERVER_HOST` | Server bind address | No | `0.0.0.0` |
| `SERVER_PORT` | Server port | No | `8081` |
| `KEYCLOAK_URL` | Keycloak server URL | Yes | - |
| `KEYCLOAK_REALM` | Keycloak realm name | Yes | - |
| `KEYCLOAK_CLIENT_ID` | Keycloak client ID | Yes | - |
| `KEYCLOAK_CLIENT_SECRET` | Keycloak client secret | Yes | - |
| `KEYCLOAK_ADMIN_USERNAME` | Keycloak admin username | Yes | - |
| `KEYCLOAK_ADMIN_PASSWORD` | Keycloak admin password | Yes | - |
| `RUST_LOG` | Log level | No | `info` |

## Error Handling

The service uses custom error types with proper HTTP status codes:

- `400 Bad Request` - Validation errors
- `401 Unauthorized` - Authentication failures
- `404 Not Found` - Resource doesn't exist
- `409 Conflict` - User already exists
- `500 Internal Server Error` - Database or internal errors
- `502 Bad Gateway` - Keycloak communication errors

## Technology Stack

- **actix-web** - Web framework
- **sqlx** - Async SQL toolkit
- **utoipa** - OpenAPI/Swagger generation
- **reqwest** - HTTP client for Keycloak
- **serde** - Serialization/deserialization
- **thiserror** - Error handling
- **anyhow** - Error context
- **dotenvy** - Environment variables
- **validator** - Input validation
- **chrono** - Date/time handling
- **uuid** - UUID generation
- **jsonwebtoken** - JWT token validation (optional)

## Development

### Running Tests
```bash
cargo test
```

### Checking Code
```bash
cargo fmt
cargo clippy
cargo check
```

## Production Deployment

1. Use environment-specific `.env` files
2. Enable HTTPS/TLS
3. Use a production-grade Keycloak instance
4. Set up database connection pooling
5. Configure proper logging and monitoring
6. Use secrets management for sensitive data
7. Enable rate limiting
8. Set up database backups

## License

MIT