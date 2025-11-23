auth-service/
├── Cargo.toml
├── Cargo.lock
├── .env
├── .env.example
├── docker-compose.yml
├── Dockerfile
├── README.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── role.rs
│   │   │   ├── permission.rs
│   │   │   └── session.rs
│   │   ├── value_objects/
│   │   │   ├── mod.rs
│   │   │   ├── email.rs
│   │   │   ├── password.rs
│   │   │   └── user_id.rs
│   │   ├── entities/
│   │   │   ├── mod.rs
│   │   │   ├── user_entity.rs
│   │   │   └── auth_session.rs
│   │   ├── events/
│   │   │   ├── mod.rs
│   │   │   ├── user_events.rs
│   │   │   └── auth_events.rs
│   │   └── services/
│   │       ├── mod.rs
│   │       ├── auth_service.rs
│   │       └── user_service.rs
│   ├── application/
│   │   ├── mod.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── user_commands.rs
│   │   │   └── auth_commands.rs
│   │   ├── queries/
│   │   │   ├── mod.rs
│   │   │   ├── user_queries.rs
│   │   │   └── auth_queries.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── command_handlers.rs
│   │   │   └── query_handlers.rs
│   │   ├── dto/
│   │   │   ├── mod.rs
│   │   │   ├── requests/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── auth_requests.rs
│   │   │   │   └── user_requests.rs
│   │   │   └── responses/
│   │   │       ├── mod.rs
│   │   │       ├── auth_responses.rs
│   │   │       └── user_responses.rs
│   │   └── ports/
│   │       ├── mod.rs
│   │       ├── repositories.rs
│   │       └── external_auth.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   ├── app_config.rs
│   │   │   └── keycloak_config.rs
│   │   ├── persistence/
│   │   │   ├── mod.rs
│   │   │   ├── repositories/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── user_repository.rs
│   │   │   │   └── session_repository.rs
│   │   │   └── migrations/
│   │   │       └── (if using local database)
│   │   ├── external/
│   │   │   ├── mod.rs
│   │   │   ├── keycloak/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── client.rs
│   │   │   │   ├── models.rs
│   │   │   │   └── adapter.rs
│   │   │   └── http_client.rs
│   │   ├── security/
│   │   │   ├── mod.rs
│   │   │   ├── jwt.rs
│   │   │   ├── crypto.rs
│   │   │   └── middleware.rs
│   │   └── logging/
│   │       ├── mod.rs
│   │       └── logger.rs
│   ├── interfaces/
│   │   ├── mod.rs
│   │   ├── controllers/
│   │   │   ├── mod.rs
│   │   │   ├── auth_controller.rs
│   │   │   └── user_controller.rs
│   │   ├── routes/
│   │   │   ├── mod.rs
│   │   │   ├── auth_routes.rs
│   │   │   └── user_routes.rs
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── auth_middleware.rs
│   │       └── logging_middleware.rs
│   └── shared/
│       ├── mod.rs
│       ├── error.rs
│       ├── result.rs
│       └── constants.rs
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── auth_test.rs
│   │   └── user_test.rs
│   ├── unit/
│   │   ├── mod.rs
│   │   ├── domain/
│   │   └── application/
│   └── helpers/
│       ├── mod.rs
│       └── test_utils.rs
├── migrations/
│   └── (database migrations if needed)
├── scripts/
│   ├── setup_keycloak.sh
│   └── deploy.sh
└── docs/
    ├── api.md
    └── architecture.md





Authentication Endpoints
User Registration & Login
text

POST /api/v1/auth/register              # Register new user
POST /api/v1/auth/login                 # User login
POST /api/v1/auth/logout                # User logout
POST /api/v1/auth/refresh               # Refresh access token
POST /api/v1/auth/verify-email          # Verify email address
POST /api/v1/auth/resend-verification   # Resend email verification

Password Management
text

POST /api/v1/auth/forgot-password       # Request password reset
POST /api/v1/auth/reset-password        # Reset password with token
POST /api/v1/auth/change-password       # Change password (authenticated)

Social & OAuth2
text

GET  /api/v1/auth/oauth/{provider}      # Initiate OAuth flow (google, github, etc.)
POST /api/v1/auth/oauth/callback        # OAuth callback handler
POST /api/v1/auth/link-social           # Link social account to existing user
POST /api/v1/auth/unlink-social         # Unlink social account

User Management Endpoints
User Profile
text

GET    /api/v1/users/me                 # Get current user profile
PUT    /api/v1/users/me                 # Update current user profile
PATCH  /api/v1/users/me                 # Partial update user profile
DELETE /api/v1/users/me                 # Delete current user account

User Administration (Admin only)
text

GET    /api/v1/admin/users              # List all users (with pagination)
GET    /api/v1/admin/users/{id}         # Get user by ID
POST   /api/v1/admin/users              # Create user (admin)
PUT    /api/v1/admin/users/{id}         # Update user
DELETE /api/v1/admin/users/{id}         # Delete user
PATCH  /api/v1/admin/users/{id}/status  # Update user status (enable/disable)

Role & Permission Management
Roles
text

GET    /api/v1/roles                   # List all roles
GET    /api/v1/roles/{id}              # Get role details
POST   /api/v1/roles                   # Create new role (admin)
PUT    /api/v1/roles/{id}              # Update role
DELETE /api/v1/roles/{id}              # Delete role

User Roles
text

GET    /api/v1/users/{id}/roles        # Get user roles
POST   /api/v1/users/{id}/roles        # Assign roles to user
DELETE /api/v1/users/{id}/roles/{role} # Remove role from user

Permissions
text

GET    /api/v1/permissions             # List all permissions
GET    /api/v1/roles/{id}/permissions  # Get role permissions
POST   /api/v1/roles/{id}/permissions  # Assign permissions to role
DELETE /api/v1/roles/{id}/permissions  # Remove permissions from role

Session Management
text

GET    /api/v1/sessions                # Get user's active sessions
GET    /api/v1/sessions/{id}           # Get specific session details
DELETE /api/v1/sessions/{id}           # Terminate specific session
DELETE /api/v1/sessions                # Terminate all sessions (except current)

Security & MFA
Multi-Factor Authentication
text

POST   /api/v1/mfa/setup               # Start MFA setup
POST   /api/v1/mfa/verify              # Verify MFA setup
POST   /api/v1/mfa/authenticate        # MFA login verification
DELETE /api/v1/mfa                     # Disable MFA
POST   /api/v1/mfa/recovery-codes      # Generate new recovery codes

Security Settings
text

GET    /api/v1/security/settings       # Get security settings
PUT    /api/v1/security/settings       # Update security settings

Client Management (Keycloak Clients)
text

GET    /api/v1/admin/clients           # List all clients
GET    /api/v1/admin/clients/{id}      # Get client details
POST   /api/v1/admin/clients           # Create new client
PUT    /api/v1/admin/clients/{id}      # Update client
DELETE /api/v1/admin/clients/{id}      # Delete client

Realm Management
text

GET    /api/v1/admin/realms            # List available realms
GET    /api/v1/admin/realms/{name}     # Get realm details
POST   /api/v1/admin/realms            # Create new realm
PUT    /api/v1/admin/realms/{name}     # Update realm
DELETE /api/v1/admin/realms/{name}     # Delete realm

Health & Monitoring
text

GET    /health                         # Health check
GET    /health/ready                   # Readiness check
GET    /health/live                    # Liveness check
GET    /metrics                        # Application metrics

WebSocket & Real-time
text

GET    /ws/notifications               # WebSocket for real-time notifications

Request/Response Examples
