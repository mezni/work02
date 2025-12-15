i need auth-service rust DDD, use anyhow, thiserror, swagger support  , tracing , tracing-subscriber , dotenvy, actix-web, actix-cors, sqlx with postgres support, utoipa and utoipa-swagger-ui, files core/logging.rs core/config.rs core/errors.rs core/database, and directories: domain, application, infrastructure and interfaces, i have -- Users table with updated schema
CREATE TABLE IF NOT EXISTS users (
    user_id VARCHAR(32) PRIMARY KEY,           -- USR + nanoid
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    photo TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
role VARCHAR(50) NOT NULL DEFAULT 'USER',
    network_id VARCHAR(32) NOT NULL DEFAULT '',
    station_id VARCHAR(32) NOT NULL DEFAULT '',
    source VARCHAR(20) NOT NULL DEFAULT 'web',   -- 'web' or 'internal'
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    created_by VARCHAR(32),
    updated_by VARCHAR(32),
CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'partner', 'operator')),
CONSTRAINT valid_source CHECK (source IN ('web', 'internal'))
); and my config DATABASE_URL=postgresql://postgres:password@localhost:6200/auth_db
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info
# Keycloak Configuration
KEYCLOAK_URL=http://localhost:5080
KEYCLOAK_REALM=myrealm
# Frontend client (public, for user authentication)
KEYCLOAK_AUTH_CLIENT_ID=auth-client
# Backend service account (confidential, for admin operations)
KEYCLOAK_BACKEND_CLIENT_ID=backend-admin
KEYCLOAK_BACKEND_CLIENT_SECRET=backend-admin-secret,, token will contains roles and attributes network_id and station_id ,  external users can self-register role = user, network_id = X and station_id = X and source = web an email of validation will be sent, for admin users can create others users with roles in operator, partner and admin, and network_id  and source = internal, if role is partner and network_id , station_id when role = operator , i need to synchronise keycloak and DB,  i want to audit users operations with their geographic locations and use nanoid for ids

auth-service/
├── Cargo.toml
├── Cargo.lock
├── .env
├── README.md
├── lib.rs
├── main.rs
├── migrations/            
│   ├── 001_create_users.sql
│   ├── 002_create_audits.sql
│   ├── 003_create_outbox_events.sql
│   └── 004_indexes.sql
├── core/                  
│   ├── config.rs
│   ├── errors.rs
│   ├── logging.rs
│   ├── database.rs
│   ├── id_generator.rs
│   └── jwt.rs
├── domain/               
│   ├── user.rs
│   ├── audit.rs
│   ├── value_objects.rs
│   └── events.rs
├── application/           
│   ├── user_commands.rs
│   ├── user_queries.rs
│   └── dtos.rs
├── infrastructure/        
│   ├── repositories/
│   │   ├── user_repository.rs
│   │   ├── audit_repository.rs
│   │   └── outbox_repository.rs
│   ├── keycloak_client.rs
│   └── cache.rs
└── interfaces/            
    └── http/
        ├── handlers.rs
        ├── routes.rs
        ├── middleware.rs
        └── docs.rs



Project Summary: Auth-Service

Architecture:

Domain-Driven Design (DDD)

Domain: Entities (User, Audit), value objects, events

Application: Commands (create/update/delete user), queries (get/search user), DTOs

Infrastructure: Repositories (Postgres via SQLx), Keycloak client, in-memory cache, outbox events

Interfaces/HTTP: Actix-web handlers, routes, middleware, Swagger/OpenAPI docs

Authentication & Authorization:

JWT-based auth with RS256 (asymmetric keys)

JWT includes DB user_id (sub), roles, network_id, station_id

Microservices verify JWT using public key

Short-lived tokens, no refresh token

RBAC/ABAC enforcement: roles + attributes (network_id, station_id)

User Management:

Self-registration: external users (role=user, source=web)

Admin operations: create/update/delete users (admin, partner, operator)

Soft delete: is_active = false instead of permanent deletion

Email verification via Keycloak

Keycloak Sync:

Bi-directional sync between DB and Keycloak

Minimal Keycloak hits with in-memory caching

Passwords managed only in Keycloak

Audit & Logging:

Log all user operations and login attempts

Includes IP address and User-Agent

Optionally extendable for geolocation

Swagger & API Docs:

All endpoints under /api/v1/...

Swagger UI at /swagger-ui/

Annotated DTOs for input/output

Other Features:

Health check endpoint /api/v1/health

Current user info /api/v1/me


# Auth-Service

## Overview

`auth-service` is a Rust-based authentication and authorization microservice built with **DDD principles**, **JWT authentication**, and **Keycloak integration**. It manages users, roles, and audits all operations.

### Features

- **JWT Authentication** (RS256) with short-lived tokens
- **Role-based (RBAC) and attribute-based (ABAC) authorization**
- **User Management**
  - Self-registration for external users (`role=user`)
  - Admin-created users (`admin`, `partner`, `operator`)
  - Soft delete (`is_active=false`) for user deletion
  - Email verification via Keycloak
- **Audit Logging**
  - Logs login attempts and all user operations
  - Captures IP and User-Agent
- **Keycloak Synchronization**
  - Bi-directional sync with DB
  - In-memory caching to minimize hits
  - Password management handled entirely in Keycloak
- **Swagger/OpenAPI documentation** at `/swagger-ui/`
- **Health check endpoint** `/api/v1/health`
- **Public key endpoint** `/api/v1/public-key` for microservices to validate JWTs

---

## Architecture


- DDD layered architecture ensures separation of concerns.
- JWT tokens are issued by Keycloak.
- Soft delete preserves data integrity for audits and historical records.

---

## Endpoints

| Method | Path | Description | Auth |
|--------|------|------------|------|
| POST   | `/api/v1/login`        | Login via Keycloak, return JWT | No |
| POST   | `/api/v1/register`     | Self-register external user | No |
| GET    | `/api/v1/verify-email`| Email verification | No |
| POST   | `/api/v1/users`        | Admin creates user | Yes |
| GET    | `/api/v1/users/{user_id}` | Get user | Yes |
| GET    | `/api/v1/users`        | Search users | Yes |
| PUT    | `/api/v1/users/{user_id}` | Update user | Yes |
| DELETE | `/api/v1/users/{user_id}` | Soft delete user | Yes |
| GET    | `/api/v1/audits`       | Search audit logs | Yes |
| GET    | `/api/v1/logins`       | Get login attempts | Yes |
| GET    | `/api/v1/health`       | Health check | No |
| GET    | `/api/v1/me`           | Current user info from JWT | Yes |

---

## JWT Verification

- Microservices verify JWTs **issued by Keycloak** using Keycloak’s **public keys** (JWKS endpoint):




- Ensure Keycloak JWT includes:
  - `sub` → DB `user_id`
  - `roles`
  - `network_id`
  - `station_id`

- Auth-service `/me` endpoint can be used to resolve `sub` → DB user info and check `is_active`.

---

## Configuration

Environment variables (`.env`):

```env
DATABASE_URL=postgresql://postgres:password@localhost:6200/auth_db
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info

# Keycloak
KEYCLOAK_URL=http://localhost:5080
KEYCLOAK_REALM=myrealm
KEYCLOAK_AUTH_CLIENT_ID=auth-client
KEYCLOAK_BACKEND_CLIENT_ID=backend-admin
KEYCLOAK_BACKEND_CLIENT_SECRET=backend-admin-secret

