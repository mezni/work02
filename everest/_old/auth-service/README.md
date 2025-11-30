# docker-compose.yml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    container_name: auth_postgres
    environment:
      POSTGRES_USER: auth_user
      POSTGRES_PASSWORD: auth_password
      POSTGRES_DB: auth_db
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U auth_user"]
      interval: 10s
      timeout: 5s
      retries: 5

  keycloak:
    image: quay.io/keycloak/keycloak:23.0
    container_name: auth_keycloak
    environment:
      KEYCLOAK_ADMIN: admin
      KEYCLOAK_ADMIN_PASSWORD: admin
      KC_DB: postgres
      KC_DB_URL: jdbc:postgresql://postgres:5432/keycloak_db
      KC_DB_USERNAME: auth_user
      KC_DB_PASSWORD: auth_password
      KC_HOSTNAME: localhost
      KC_HTTP_ENABLED: "true"
    command: start-dev
    ports:
      - "8080:8080"
    depends_on:
      postgres:
        condition: service_healthy

  auth-service:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: auth_service
    environment:
      DATABASE__URL: postgresql://auth_user:auth_password@postgres:5432/auth_db
      DATABASE__MAX_CONNECTIONS: 10
      KEYCLOAK__URL: http://keycloak:8080
      KEYCLOAK__REALM: auth-service
      KEYCLOAK__CLIENT_ID: auth-service-client
      KEYCLOAK__CLIENT_SECRET: your-client-secret
      KEYCLOAK__ADMIN_USERNAME: admin
      KEYCLOAK__ADMIN_PASSWORD: admin
      SERVER__HOST: 0.0.0.0
      SERVER__PORT: 8000
      RUST_LOG: info
    ports:
      - "8000:8000"
    depends_on:
      postgres:
        condition: service_healthy
      keycloak:
        condition: service_started
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  postgres_data:

---
# Dockerfile
FROM rust:1.90-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/auth-service /app/auth-service
COPY --from=builder /app/migrations /app/migrations

# Set environment
ENV RUST_LOG=info

EXPOSE 8000

CMD ["./auth-service"]

---
# .dockerignore
target/
.git/
.env
*.log
coverage/

---
# README.md
# Auth-Service - DDD Microservice with Keycloak Integration

A production-ready authentication and authorization microservice built with Rust, following Domain-Driven Design (DDD) principles.

## Features

- **Domain-Driven Design Architecture**
  - Pure domain layer with no external dependencies
  - Clear separation of concerns across all layers
  - Repository pattern for data access
  - Domain services for business logic

- **Role-Based Access Control**
  - Admin: Full system access
  - Partner: Organisation-level access, can manage operators
  - Operator: Station-level access within a partner organisation

- **Keycloak Integration**
  - User creation and management
  - Role assignment
  - JWT token authentication
  - Token enrichment with organisation data

- **PostgreSQL Database**
  - Connection pooling with SQLx
  - Automatic migrations
  - Optimized indexes for performance

- **OpenAPI/Swagger Documentation**
  - Interactive API documentation at `/swagger-ui`
  - Auto-generated from code annotations

- **Observability**
  - Structured logging with tracing
  - Request/response logging middleware
  - Health check endpoint

## Architecture

```
src/
├── domain/              # Pure domain logic
│   ├── entities.rs      # User entity
│   ├── value_objects.rs # Email, Role, OrganisationName
│   ├── repositories.rs  # Repository traits
│   ├── services.rs      # Domain services
│   └── errors.rs        # Domain errors
├── application/         # Application layer
│   ├── dto.rs          # Data Transfer Objects
│   ├── commands.rs     # Command structures
│   ├── queries.rs      # Query structures
│   ├── handlers.rs     # Command/Query handlers
│   └── errors.rs       # Application errors
├── infrastructure/      # External integrations
│   ├── keycloak/       # Keycloak client
│   ├── repositories/   # SQLx implementations
│   ├── config.rs       # Configuration
│   └── jwt.rs          # JWT utilities
├── interfaces/          # HTTP layer
│   ├── http/           # HTTP handlers and routes
│   ├── middleware/     # Auth and logging middleware
│   └── api_doc.rs      # OpenAPI documentation
└── main.rs             # Application entry point
```

## Prerequisites

- Rust 1.90+
- PostgreSQL 15+
- Keycloak 23+
- Docker and Docker Compose (optional)

## Quick Start

### Using Docker Compose

1. Clone the repository
2. Run the services:
```bash
docker-compose up -d
```

3. Access the services:
   - Auth Service: http://localhost:8000
   - Swagger UI: http://localhost:8000/swagger-ui
   - Keycloak Admin: http://localhost:8080

### Manual Setup

1. Install dependencies:
```bash
cargo build
```

2. Setup PostgreSQL and Keycloak

3. Configure environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Run migrations:
```bash
sqlx migrate run
```

5. Start the service:
```bash
cargo run
```

## Configuration

Create a `.env` file with the following variables:

```env
# Database
DATABASE__URL=postgresql://auth_user:auth_password@localhost:5432/auth_db
DATABASE__MAX_CONNECTIONS=10

# Keycloak
KEYCLOAK__URL=http://localhost:8080
KEYCLOAK__REALM=auth-service
KEYCLOAK__CLIENT_ID=auth-service-client
KEYCLOAK__CLIENT_SECRET=your-client-secret
KEYCLOAK__ADMIN_USERNAME=admin
KEYCLOAK__ADMIN_PASSWORD=admin

# Server
SERVER__HOST=0.0.0.0
SERVER__PORT=8000
```

## API Endpoints

### Authentication

- `POST /api/v1/auth/login` - User login
  ```json
  {
    "username": "admin",
    "password": "password"
  }
  ```

### User Management

- `POST /api/v1/users` - Create user (requires authentication)
  ```json
  {
    "email": "user@example.com",
    "username": "newuser",
    "password": "password123",
    "role": "operator",
    "organisation_name": "AcmeCorp"
  }
  ```

- `GET /api/v1/users/{id}` - Get user by ID
- `GET /api/v1/organisations/{org_name}/users` - List users by organisation

### Health Check

- `GET /health` - Service health status

## Testing

Run unit tests:
```bash
cargo test --lib
```

Run integration tests:
```bash
cargo test --test '*' -- --test-threads=1
```

Run all tests with coverage:
```bash
cargo tarpaulin --out Html --output-dir coverage
```

## Performance Characteristics

- Designed for ~2000 users
- Handles ~5 req/sec average load
- Supports ~50 concurrent users
- PostgreSQL connection pooling
- Optimized database indexes

## Security Features

- JWT-based authentication via Keycloak
- Role-based access control (RBAC)
- Organisation-scoped data access
- Password validation
- Secure credential storage in Keycloak

## Development

### Adding a New Endpoint

1. Define DTOs in `application/dto.rs`
2. Create command/query in `application/commands.rs` or `queries.rs`
3. Implement handler in `application/handlers.rs`
4. Add HTTP handler in `interfaces/http/handlers.rs`
5. Register route in `interfaces/http/routes.rs`
6. Update OpenAPI docs in handler annotations

### Database Migrations

Create a new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

## Monitoring and Logging

Logs are output in structured format. Configure log level via `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

Available log levels: trace, debug, info, warn, error

## Contributing

1. Follow DDD principles
2. Keep domain layer pure (no external dependencies)
3. Write tests for new features
4. Update OpenAPI documentation
5. Run `cargo fmt` and `cargo clippy` before committing

## License

MIT License