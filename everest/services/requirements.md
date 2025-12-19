Health & Status
    GET /health: Service health check (no auth)
    GET /metrics: Prometheus metrics (no auth)
    GET /version: API version info (no auth)

Registration & Verification
    POST /register: Create new registration (no auth)
    POST /verify: Verify registration token (no auth)
    POST /verify/resend: Resend verification email (no auth)
    GET /register/{id}: Get registration status (no auth)
    DELETE /register/{id}: Cancel registration (no auth)

Authentication
    POST /auth/login: User login (no auth)
    POST /auth/logout: User logout (auth required)
    POST /auth/refresh: Refresh tokens (no auth)
    POST /auth/validate: Validate token (no auth)
    GET /auth/userinfo: Get user info (auth required)
    GET /auth/sessions: List active sessions (auth required)
    DELETE /auth/sessions/{id}: Revoke session (auth required)

Password Management
    POST /password/reset/request: Request password reset (no auth)
    POST /password/reset/confirm: Confirm password reset (no auth)
    POST /password/change: Change password (auth required)
    GET /password/policy: Get password policy (no auth)

User Profile

    GET /users/me: Get current user profile (auth required)
    PUT /users/me: Update user profile (auth required)
    PATCH /users/me: Partial update (auth required)
    GET /users/me/preferences: Get user preferences (auth required)
    PUT /users/me/preferences: Update preferences (auth required)
    GET /users/me/audit: Get user audit logs (auth required)
    DELETE /users/me: Delete own account (auth required)

Admin Endpoints

    GET /admin/users: List all users (admin auth)
    GET /admin/users/{id}: Get user by ID (admin auth)
    POST /admin/users: Create user (admin auth)
    PUT /admin/users/{id}: Update user (admin auth)
    DELETE /admin/users/{id}: Delete user (admin auth)
    ... (rest of admin endpoints)

Invitations

    POST /invitations: Create invitation (admin auth)
    GET /invitations: List invitations (admin auth)
    GET /invitations/{code}: Get invitation (no auth)
    POST /invitations/{code}/accept: Accept invitation (no auth)
    DELETE /invitations/{code}: Cancel invitation (admin auth)

Security

    GET /security/mfa/setup: Setup MFA (auth required)
    POST /security/mfa/verify: Verify MFA (auth required)
    DELETE /security/mfa: Disable MFA (auth required)
    GET /security/devices: List trusted devices (auth required)
    DELETE /security/devices/{id}: Remove device (auth required)

Analytics

    GET /analytics/registrations: Registration stats (admin auth)
    GET /analytics/logins: Login statistics (admin auth)
    GET /analytics/active-users: Active user count (admin auth)

System

    GET /system/config: Public config (no auth)
    POST /system/webhooks/keycloak: Keycloak webhook (no auth)
    GET /system/status: System status (admin auth)

Want me to summarize a specific section or provide more details on an endpoint? 😊







#------------------------------------------


    Files:
        main.rs
        lib.rs
        core/config.rs
        core/logging.rs
        core/errors.rs
    Libraries:
        actix-web
        tokio
        thiserror
        anyhow
        tracing
        tracing-subscriber
    Functionality:
        Actix-web application setup
        CORS (Cross-Origin Resource Sharing) setup
        Application state management in lib.rs
        Logging setup using tracing and tracing-subscriber
        Error handling using thiserror and anyhow
        keep main minimal

give me a modern rust code 

Health Check
    GET /api/v1/health - Check API service status

Registration Flow
    POST /api/v1/register - Initial user registration (submit email/password)
    POST /api/v1/register/verify - Verify email with OTP/token
    POST /api/v1/register/resend - Resend verification code

Authentication
    POST /api/v1/auth/login - User login with credentials
    POST /api/v1/auth/logout - User logout (invalidate session/token)

Recommended Additional Endpoints for Self-Registration:
User Profile Management
    GET /api/v1/users/me - Get current user profile
    PUT /api/v1/users/me - Update user profile
    PATCH /api/v1/users/me/password - Change password


Account Recovery
    POST /api/v1/auth/forgot-password - Request password reset
    POST /api/v1/auth/reset-password - Reset password with token


Session Management
    POST /api/v1/auth/refresh - Refresh access token
    GET /api/v1/auth/sessions - List active sessions
    DELETE /api/v1/auth/sessions/{id} - Revoke specific session

Optional: Account Management
    DELETE /api/v1/users/me - Delete user account (soft/hard delete)
    POST /api/v1/users/me/deactivate - Deactivate account temporarily
    POST /api/v1/register/check-email - Check email availability (pre-registration)





sequenceDiagram
    participant C as Client (Frontend)
    participant A as Auth Service
    participant K as Keycloak
    participant DB as Service Database
    participant J as Job/Cron Service

    %% ====================
    %% REGISTRATION FLOW
    %% ====================
    Note over C,A: 1. REGISTRATION FLOW
    C->>A: POST /api/v1/register
    A->>K: Check if user exists
    K-->>A: User not found
    A->>DB: Create user record (status: "pending_verification")
    DB-->>A: User saved
    A->>K: Create user (enabled: false, emailVerified: false)
    K-->>A: User created in Keycloak
    A->>K: Send verification email
    A->>DB: Set verification expiry (24h)
    A-->>C: 201 Created - Verification email sent

    %% ====================
    %% VERIFICATION FLOWS
    %% ====================
    
    %% Successful verification
    Note over C,A: 2. EMAIL VERIFICATION (SUCCESS)
    C->>A: POST /api/v1/register/verify
    A->>DB: Check verification expiry
    DB-->>A: Still valid (<24h)
    A->>K: Verify email token/OTP
    K-->>A: Email verified
    A->>K: Enable user account
    A->>DB: Update user status to "active"
    A-->>C: 200 OK - Account activated

    %% Expired verification
    Note over C,A: 3. EMAIL VERIFICATION (EXPIRED)
    C->>A: POST /api/v1/register/verify
    A->>DB: Check verification expiry
    DB-->>A: Verification expired (>24h)
    A->>DB: Update status to "verification_expired"
    A->>K: Delete temporary user
    A-->>C: 410 Gone - Verification expired

    %% Resend verification
    Note over C,A: 4. RESEND VERIFICATION
    C->>A: POST /api/v1/register/resend
    A->>DB: Check last sent time & expiry
    DB-->>A: Can resend (within limits)
    A->>K: Resend verification email
    K-->>A: Email sent
    A->>DB: Update resend count & timestamp
    A-->>C: 200 OK - New verification sent

    %% ====================
    %% AUTHENTICATION FLOWS
    %% ====================

    %% Successful login (verified user)
    Note over C,A: 5. LOGIN FLOW (VERIFIED USER)
    C->>A: POST /api/v1/auth/login
    A->>K: Authenticate user (OAuth2/OpenID)
    K-->>A: Return tokens (access, refresh, id_token)
    A->>DB: Store session/refresh token
    A-->>C: 200 OK with tokens

    %% Login attempt for unverified user
    Note over C,A: 6. LOGIN FLOW (UNVERIFIED USER)
    C->>A: POST /api/v1/auth/login
    A->>K: Authenticate user
    K-->>A: Authentication successful but user disabled
    A->>DB: Check user status
    DB-->>A: Status = "pending_verification"
    A-->>C: 403 Forbidden - Account not verified

    %% ====================
    %% TOKEN MANAGEMENT
    %% ====================

    %% Token validation for API access
    Note over C,A: 7. TOKEN VALIDATION (API ACCESS)
    C->>A: API Request with Bearer token
    A->>K: Introspect token / Validate JWT
    K-->>A: Token valid + user info
    A->>DB: Get user permissions/roles
    A-->>C: Proceed with business logic

    %% Refresh token flow
    Note over C,A: 8. TOKEN REFRESH FLOW
    C->>A: POST /api/v1/auth/refresh
    A->>DB: Validate refresh token
    DB-->>A: Token valid
    A->>K: Refresh token grant
    K-->>A: New access token
    A->>DB: Update session
    A-->>C: New tokens

    %% ====================
    %% SESSION MANAGEMENT
    %% ====================

    %% Logout flow
    Note over C,A: 9. LOGOUT FLOW
    C->>A: POST /api/v1/auth/logout
    A->>K: Revoke tokens
    K-->>A: Tokens revoked
    A->>DB: Invalidate session
    A->>K: Logout user session
    A-->>C: 200 OK

    %% ====================
    %% ALTERNATIVE FLOWS
    %% ====================

    %% Direct Keycloak integration
    Note over C,K: 10. DIRECT KEYCLOAK FLOW (Alternative)
    C->>K: Redirect to Keycloak login
    K->>C: Return authorization code
    C->>A: Send code for token exchange
    A->>K: Exchange code for tokens
    K-->>A: Return tokens
    A-->>C: Tokens for API access

    %% ====================
    %% MAINTENANCE FLOWS
    %% ====================

    %% Cleanup job for expired registrations
    Note over J,A: 11. CLEANUP PROCESS (DAILY JOB)
    J->>DB: Find expired pending verifications
    DB-->>J: List of expired users
    loop For each expired user
        J->>K: Delete user from Keycloak
        J->>DB: Mark as "cleaned_up"
    end

    %% Re-registration after expiry
    Note over C,A: 12. RE-REGISTRATION AFTER EXPIRY
    C->>A: POST /api/v1/register (same email)
    A->>DB: Check previous records
    DB-->>A: Previous status = "verification_expired"
    A->>K: Check if user exists
    K-->>A: User not found (was deleted)
    A->>DB: Create new registration record
    A->>K: Create new temporary user
    A-->>C: 201 Created - New verification sent



classDiagram
    %% ====================
    %% ENUMERATIONS
    %% ====================
    class KeycloakError {
        <<enumeration>>
        +Authentication(String)
        +TokenValidation(String)
        +UserManagement(String)
        +Network(String)
        +Config(String)
        +display(): String
    }

    %% ====================
    %% CONFIGURATION
    %% ====================
    class KeycloakConfig {
        -keycloak_url: String
        -keycloak_realm: String
        -auth_client_id: String
        -auth_client_redirect_uri: String
        -backend_client_id: String
        -backend_client_secret: String
        -token_expiry_buffer_seconds: i64
        -admin_token_refresh_interval: u64
        +from_env() KeycloakConfig
        +get_keycloak_url() String
        +get_keycloak_realm() String
        +get_auth_client_id() String
        +get_backend_client_id() String
        +get_backend_client_secret() String
    }

    %% ====================
    %% DATA STRUCTURES
    %% ====================
    class AdminToken {
        -access_token: String
        -refresh_token: Option~String~
        -expires_at: DateTime~Utc~
        -token_type: String
        -scope: Option~String~
        +is_expired() bool
        +get_access_token() String
        +get_refresh_token() Option~String~
    }

    class UserInfo {
        +sub: String
        +email: String
        +email_verified: bool
        +preferred_username: Option~String~
        +given_name: Option~String~
        +family_name: Option~String~
        +enabled: bool
        +is_active() bool
    }

    class CreateUserRequest {
        +username: String
        +email: String
        +enabled: bool
        +email_verified: bool
        +credentials: Vec~Credential~
        +attributes: Option~serde_json::Value~
        +new(username, email, password) CreateUserRequest
        +add_attribute(key, value)
    }

    class Credential {
        +type: String
        +value: String
        +temporary: bool
        +new_password(value) Credential
        +new_totp(value) Credential
    }

    class TokenResponse {
        <<serializable>>
        +access_token: String
        +refresh_token: Option~String~
        +expires_in: i64
        +refresh_expires_in: i64
        +token_type: String
        +scope: Option~String~
        +id_token: Option~String~
    }

    %% ====================
    %% MAIN CLIENT CLASS
    %% ====================
    class KeycloakClient {
        -http: reqwest::Client
        -config: KeycloakConfig
        -admin_token: Arc~RwLock~Option~AdminToken~~~
        +new(config: KeycloakConfig) KeycloakClient
        +with_custom_http(client: reqwest::Client) KeycloakClient
    }

    %% ====================
    %% TRAIT INTERFACES
    %% ====================
    class IAdminOperations {
        <<interface>>
        +create_user(user: CreateUserRequest) Result~String, KeycloakError~
        +get_user_by_email(email: &str) Result~Option~UserInfo~, KeycloakError~
        +update_user(user_id: &str, update: UserUpdate) Result~(), KeycloakError~
        +delete_user(user_id: &str) Result~(), KeycloakError~
        +execute_email_verification(user_id: &str) Result~(), KeycloakError~
        +send_actions_email(user_id: &str, actions: Vec~String~) Result~(), KeycloakError~
        +list_users(query: UserQuery) Result~Vec~UserInfo~, KeycloakError~
    }

    class IAuthentication {
        <<interface>>
        +authenticate_user(username: &str, password: &str) Result~TokenResponse, KeycloakError~
        +validate_token(token: &str) Result~UserInfo, KeycloakError~
        +refresh_token(refresh_token: &str) Result~TokenResponse, KeycloakError~
        +logout(refresh_token: &str) Result~(), KeycloakError~
        +introspect_token(token: &str) Result~TokenInfo, KeycloakError~
    }

    class ITokenManager {
        <<interface>>
        +get_admin_token() Result~String, KeycloakError~
        +fetch_admin_token() Result~AdminToken, KeycloakError~
        +clear_admin_token()
        +is_admin_token_valid() bool
        +schedule_token_refresh()
    }

    %% ====================
    %% SUPPORTING CLASSES
    %% ====================
    class UserQuery {
        +email: Option~String~
        +username: Option~String~
        +first_name: Option~String~
        +last_name: Option~String~
        +enabled: Option~bool~
        +email_verified: Option~bool~
        +first: Option~i32~
        +max: Option~i32~
        +to_query_params() Vec~(String, String)~
    }

    class UserUpdate {
        +email: Option~String~
        +enabled: Option~bool~
        +email_verified: Option~bool~
        +attributes: Option~serde_json::Value~
        +to_json() serde_json::Value
    }

    class TokenInfo {
        +active: bool
        +sub: String
        +username: String
        +email: String
        +email_verified: bool
        +realm_access: serde_json::Value
        +resource_access: serde_json::Value
        +exp: i64
        +iat: i64
        +is_expired() bool
    }

    class EmailAction {
        <<enumeration>>
        +VERIFY_EMAIL
        +UPDATE_PASSWORD
        +UPDATE_PROFILE
        +CONFIGURE_TOTP
        +to_string() String
    }

    %% ====================
    %% BUILDER PATTERN
    %% ====================
    class KeycloakClientBuilder {
        -config: Option~KeycloakConfig~
        -http_client: Option~reqwest::Client~
        +new() KeycloakClientBuilder
        +with_config(config: KeycloakConfig) KeycloakClientBuilder
        +with_env_config() KeycloakClientBuilder
        +with_http_client(client: reqwest::Client) KeycloakClientBuilder
        +with_timeout(timeout: Duration) KeycloakClientBuilder
        +build() Result~KeycloakClient, KeycloakError~
    }

    %% ====================
    %% FACTORY
    %% ====================
    class KeycloakClientFactory {
        -instances: HashMap~String, Arc~KeycloakClient~~
        +get_instance(realm: &str) Result~Arc~KeycloakClient~, KeycloakError~
        +create_client(config: KeycloakConfig) Result~Arc~KeycloakClient~, KeycloakError~
        +clear_instance(realm: &str)
        +clear_all()
    }

    %% ====================
    %% RELATIONSHIPS
    %% ====================
    KeycloakClient --> KeycloakConfig : uses
    KeycloakClient --> AdminToken : manages
    KeycloakClient --> IAdminOperations : implements
    KeycloakClient --> IAuthentication : implements
    KeycloakClient --> ITokenManager : implements
    
    CreateUserRequest --> Credential : contains
    KeycloakClientBuilder --> KeycloakClient : builds
    KeycloakClientFactory --> KeycloakClient : creates/manages
    
    UserQuery --> KeycloakClient : used by
    UserUpdate --> KeycloakClient : used by
    
    KeycloakError <|-- KeycloakClient : raises
    
    %% Composition relationships
    KeycloakClient "1" *-- "1" KeycloakConfig
    KeycloakClient "1" *-- "1" reqwest::Client
    KeycloakClient "1" *-- "0..1" AdminToken
    
    %% Aggregation relationships
    CreateUserRequest "1" *-- "*" Credential
    
    %% Dependency relationships
    IAdminOperations ..> CreateUserRequest
    IAdminOperations ..> UserInfo
    IAuthentication ..> TokenResponse
    ITokenManager ..> AdminToken    





classDiagram
    %% Main Application Entry Point
    class App {
        +main() async
        +configure_services() AppState
        +configure_routes() HttpServer
    }
    
    %% Core Configuration
    class AppConfig {
        +database: DatabaseConfig
        +keycloak: KeycloakConfig
        +server: ServerConfig
        +auth: AuthConfig
        +smtp: Option~SmtpConfig~
        +rate_limit: RateLimitConfig
        +log_level: String
        +new() Result~Self, ConfigError~
    }
    
    class AppState {
        -db: PgPool
        -config: AppConfig
        -keycloak_client: Arc~dyn KeycloakClient~
        -user_registration_repo: Arc~dyn UserRegistrationRepository~
        -user_repo: Arc~dyn UserRepository~
        -login_audit_repo: Arc~dyn LoginAuditRepository~
        -email_service: Arc~dyn EmailService~
    }
    
    %% Domain Layer
    class User {
        -id: Uuid
        -keycloak_id: String
        -email: String
        -username: Option~String~
        -roles: Vec~String~
        -status: String
        +validate() Result~
    }
    
    class UserRegistration {
        -id: Uuid
        -email: String
        -verification_token: String
        -verification_token_expires_at: DateTime~Utc~
        -keycloak_sync_status: String
        +is_expired() bool
        +is_verified() bool
    }
    
    %% Domain Interfaces (Traits)
    <<interface>> UserRepository
    UserRepository : +create(user: &User) AppResult~User~
    UserRepository : +find_by_id(id: Uuid) AppResult~Option~User>~
    UserRepository : +find_by_email(email: &str) AppResult~Option~User>~
    UserRepository : +update(user: &User) AppResult~User~
    UserRepository : +delete(id: Uuid) AppResult~()~
    
    <<interface>> UserRegistrationRepository
    UserRegistrationRepository : +create(reg: &UserRegistration) AppResult~UserRegistration~
    UserRegistrationRepository : +find_by_token(token: &str) AppResult~Option~UserRegistration>~
    UserRegistrationRepository : +find_by_email(email: &str) AppResult~Option~UserRegistration>~
    UserRegistrationRepository : +update(reg: &UserRegistration) AppResult~UserRegistration~
    UserRegistrationRepository : +delete_expired(before: DateTime~Utc~) AppResult~u64~
    
    <<interface>> KeycloakClient
    KeycloakClient : +create_user(email, username, first_name, last_name, password) AppResult~String~ // Returns Keycloak user ID
    KeycloakClient : +enable_user(keycloak_id) AppResult~()~
    KeycloakClient : +disable_user(keycloak_id) AppResult~()~
    KeycloakClient : +assign_role(keycloak_id, role) AppResult~()~
    KeycloakClient : +verify_token(token) AppResult~serde_json::Value~ // Returns token claims
    KeycloakClient : +get_user_info(token) AppResult~serde_json::Value~ // Returns user info
    
    <<interface>> EmailService
    EmailService : +send_verification_email(to: String, token: String) AppResult~()~
    EmailService : +send_password_reset(to: String, token: String) AppResult~()~
    EmailService : +send_welcome_email(to: String, username: String) AppResult~()~
    
    %% Application Services Layer
    class UserRegistrationService {
        -user_registration_repo: Arc~dyn UserRegistrationRepository~
        -user_repo: Arc~dyn UserRepository~
        -keycloak_client: Arc~dyn KeycloakClient~
        -email_service: Arc~dyn EmailService~
        -config: AppConfig
        +register_user(command: RegisterUserCommand) AppResult~UserRegistration~
        +verify_registration(command: VerifyRegistrationCommand) AppResult~User~
        +resend_verification(email: String) AppResult~()~
    }
    
    class UserService {
        -user_repo: Arc~dyn UserRepository~
        -keycloak_client: Arc~dyn KeycloakClient~
        -login_audit_repo: Arc~dyn LoginAuditRepository~
        +get_user(id: Uuid) AppResult~User~
        +update_profile(id: Uuid, command: UpdateProfileCommand) AppResult~User~
        +change_password(id: Uuid, command: ChangePasswordCommand) AppResult~()~
        +disable_account(id: Uuid) AppResult~()~
        +list_users(query: UserQuery) AppResult~Vec~User>~
    }
    
    class AuthService {
        -user_repo: Arc~dyn UserRepository~
        -keycloak_client: Arc~dyn KeycloakClient~
        -login_audit_repo: Arc~dyn LoginAuditRepository~
        +authenticate(command: LoginCommand) AppResult~AuthResponse~
        +refresh_token(refresh_token: String) AppResult~AuthResponse~
        +logout(access_token: String) AppResult~()~
        +validate_token(token: String) AppResult~TokenClaims~
    }
    
    %% Infrastructure Layer - Repositories
    class PostgresUserRepository {
        -pool: PgPool
        +new(pool: PgPool) Self
        +with_transaction(tx, user) AppResult~User~
    }
    
    class PostgresUserRegistrationRepository {
        -pool: PgPool
        +new(pool: PgPool) Self
    }
    
    class PostgresLoginAuditRepository {
        -pool: PgPool
        +new(pool: PgPool) Self
    }
    
    class KeycloakClientImpl {
        -http_client: reqwest::Client
        -config: KeycloakConfig
        +get_admin_token() AppResult~String~
        +create_user() AppResult~String~
        +enable_user() AppResult~()~
        +verify_token() AppResult~serde_json::Value~
    }
    
    class SmtpEmailService {
        -config: SmtpConfig
        -client: lettre::AsyncSmtpTransport
        +send_email(email: Email) AppResult~()~
    }
    
    class MockEmailService {
        -sent_emails: Vec~Email~
        +get_sent_emails() Vec~Email~
    }
    
    %% HTTP Layer (Controllers/Handlers)
    class RegistrationHandler {
        -service: Arc~UserRegistrationService~
        +register(request: HttpRequest, payload: web::Json~RegisterRequest~) HttpResponse
        +verify(request: HttpRequest, payload: web::Json~VerifyRequest~) HttpResponse
        +resend_verification(request: HttpRequest, payload: web::Json~ResendRequest~) HttpResponse
    }
    
    class AuthHandler {
        -service: Arc~AuthService~
        +login(request: HttpRequest, payload: web::Json~LoginRequest~) HttpResponse
        +refresh(request: HttpRequest, payload: web::Json~RefreshRequest~) HttpResponse
        +logout(request: HttpRequest) HttpResponse
        +validate(request: HttpRequest) HttpResponse
    }
    
    class UserHandler {
        -service: Arc~UserService~
        +get_profile(request: HttpRequest) HttpResponse
        +update_profile(request: HttpRequest, payload: web::Json~UpdateProfileRequest~) HttpResponse
        +change_password(request: HttpRequest, payload: web::Json~ChangePasswordRequest~) HttpResponse
    }
    
    class AdminHandler {
        -service: Arc~UserService~
        +list_users(request: HttpRequest) HttpResponse
        +get_user(request: HttpRequest) HttpResponse
        +disable_user(request: HttpRequest) HttpResponse
        +enable_user(request: HttpRequest) HttpResponse
    }
    
    %% DTOs (Data Transfer Objects)
    class RegisterUserCommand {
        -email: String
        -username: Option~String~
        -password: String
        -confirm_password: String
        -first_name: Option~String~
        -last_name: Option~String~
        -source: String
        +validate() Result~
    }
    
    class RegisterRequest {
        -email: String
        -username: Option~String~
        -password: String
        -confirm_password: String
        -first_name: Option~String~
        -last_name: Option~String~
    }
    
    class RegisterResponse {
        -registration_id: Uuid
        -email: String
        -expires_at: DateTime~Utc~
        -message: String
    }
    
    class VerifyRequest {
        -token: String
    }
    
    class LoginRequest {
        -email: String
        -password: String
        -device_id: Option~String~
    }
    
    class AuthResponse {
        -access_token: String
        -refresh_token: String
        -expires_in: i64
        -token_type: String
        -user: UserDto
    }
    
    class UserDto {
        -id: Uuid
        -email: String
        -username: Option~String~
        -first_name: Option~String~
        -last_name: Option~String~
        -roles: Vec~String~
    }
    
    %% Middleware
    class AuthMiddleware {
        -keycloak_client: Arc~dyn KeycloakClient~
        +extract_token(request: &HttpRequest) Option~String~
        +validate_token(token: String) AppResult~TokenClaims~
    }
    
    class RateLimitMiddleware {
        -redis_client: redis::Client
        -config: RateLimitConfig
        +check_limit(key: String) AppResult~bool~
        +increment_counter(key: String) AppResult~()~
    }
    
    class LoggingMiddleware {
        +log_request(request: &ServiceRequest)
        +log_response(response: &ServiceResponse)
    }
    
    %% Error Types
    class AppError {
        -kind: ErrorKind
        -message: String
        +validation(msg: String) Self
        +authentication(msg: String) Self
        +not_found(msg: String) Self
        +conflict(msg: String) Self
        +database(err: sqlx::Error) Self
        +keycloak(msg: String) Self
        +error_response() HttpResponse
    }
    
    class ErrorKind {
        <<enumeration>>
        Validation
        Authentication
        Authorization
        NotFound
        Conflict
        Database
        Keycloak
        ExternalService
        Internal
    }
    
    %% Relationships
    App --> AppConfig : creates
    App --> AppState : configures
    App --> HttpServer : starts
    
    AppState o-- "1" AppConfig : contains
    AppState o-- "1" PgPool : manages
    AppState o-- "*" dyn Repository : holds
    
    UserRegistrationService --> "1" UserRegistrationRepository : uses
    UserRegistrationService --> "1" UserRepository : uses
    UserRegistrationService --> "1" KeycloakClient : uses
    UserRegistrationService --> "1" EmailService : uses
    
    PostgresUserRepository ..|> UserRepository : implements
    PostgresUserRegistrationRepository ..|> UserRegistrationRepository : implements
    KeycloakClientImpl ..|> KeycloakClient : implements
    SmtpEmailService ..|> EmailService : implements
    
    RegistrationHandler --> "1" UserRegistrationService : depends on
    AuthHandler --> "1" AuthService : depends on
    UserHandler --> "1" UserService : depends on
    
    AuthMiddleware --> "1" KeycloakClient : uses
    
    RegisterRequest --> RegisterUserCommand : maps to
    User --> UserDto : converts to
    
    RegistrationHandler --> RegisterRequest : accepts
    RegistrationHandler --> RegisterResponse : returns






graph TB
    subgraph "HTTP Layer (Interfaces)"
        HTTP[HTTP Server]
        RH[RegistrationHandler]
        AH[AuthHandler]
        UH[UserHandler]
        MH[Middleware Handler]
    end
    
    subgraph "Application Layer"
        US[UserRegistrationService]
        AS[AuthService]
        USVC[UserService]
    end
    
    subgraph "Domain Layer"
        UR[UserRepository trait]
        URR[UserRegistrationRepository trait]
        KC[KeycloakClient trait]
        UE[User Entity]
        URE[UserRegistration Entity]
    end
    
    subgraph "Infrastructure Layer"
        PUR[PostgresUserRepository]
        PURR[PostgresUserRegistrationRepo]
        KCI[KeycloakClientImpl]
        DB[(PostgreSQL)]
        KCK[(Keycloak)]
    end
    
    subgraph "Core Layer"
        CFG[AppConfig]
        ERR[AppError]
        LOG[Logging]
        STATE[AppState]
    end
    
    %% Dependencies
    HTTP --> RH
    HTTP --> AH
    HTTP --> UH
    HTTP --> MH
    
    RH --> US
    AH --> AS
    UH --> USVC
    
    US --> UR
    US --> URR
    US --> KC
    US --> UE
    US --> URE
    
    AS --> UR
    AS --> KC
    
    USVC --> UR
    
    PUR --> UR
    PURR --> URR
    KCI --> KC
    
    PUR --> DB
    PURR --> DB
    KCI --> KCK
    
    RH --> CFG
    US --> CFG
    AS --> CFG
    USVC --> CFG
    KCI --> CFG
    
    RH --> ERR
    US --> ERR
    PUR --> ERR
    
    HTTP --> LOG
    PUR --> LOG
    KCI --> LOG
    
    HTTP --> STATE
    US --> STATE
    AS --> STATE
    USVC --> STATE
























🌐 Comp
lete REST API Endpoints List
📋 API Version: v1

Base URL: /api/v1
🏥 Health & Status
Method	Endpoint	Description	Auth Required	Rate Limit
GET	/health	Service health check	No	None
GET	/metrics	Prometheus metrics	No	None
GET	/version	API version info	No	None
👤 Registration & Verification
Method	Endpoint	Description	Auth Required	Rate Limit
POST	/register	Create new registration	No	10/10min
POST	/verify	Verify registration token	No	20/10min
POST	/verify/resend	Resend verification email	No	5/10min
GET	/register/{id}	Get registration status	No	30/10min
DELETE	/register/{id}	Cancel registration	No	10/10min
🔐 Authentication
Method	Endpoint	Description	Auth Required	Rate Limit
POST	/auth/login	User login	No	10/10min
POST	/auth/logout	User logout	Yes	30/10min
POST	/auth/refresh	Refresh tokens	No	30/10min
POST	/auth/validate	Validate token	No	100/10min
GET	/auth/userinfo	Get user info	Yes	60/10min
GET	/auth/sessions	List active sessions	Yes	30/10min
DELETE	/auth/sessions/{id}	Revoke session	Yes	20/10min
📧 Password Management
Method	Endpoint	Description	Auth Required	Rate Limit
POST	/password/reset/request	Request password reset	No	5/10min
POST	/password/reset/confirm	Confirm password reset	No	10/10min
POST	/password/change	Change password	Yes	10/10min
GET	/password/policy	Get password policy	No	30/10min
👤 User Profile
Method	Endpoint	Description	Auth Required	Rate Limit
GET	/users/me	Get current user profile	Yes	60/10min
PUT	/users/me	Update user profile	Yes	30/10min
PATCH	/users/me	Partial update	Yes	30/10min
GET	/users/me/preferences	Get user preferences	Yes	60/10min
PUT	/users/me/preferences	Update preferences	Yes	30/10min
GET	/users/me/audit	Get user audit logs	Yes	30/10min
DELETE	/users/me	Delete own account	Yes	5/10min
👑 Admin Endpoints
Method	Endpoint	Description	Auth Required	Rate Limit
GET	/admin/users	List all users	Admin	60/10min
GET	/admin/users/{id}	Get user by ID	Admin	60/10min
POST	/admin/users	Create user	Admin	30/10min
PUT	/admin/users/{id}	Update user	Admin	30/10min
DELETE	/admin/users/{id}	Delete user	Admin	10/10min
POST	/admin/users/{id}/enable	Enable user	Admin	20/10min
POST	/admin/users/{id}/disable	Disable user	Admin	20/10min
POST	/admin/users/{id}/lock	Lock user	Admin	20/10min
POST	/admin/users/{id}/unlock	Unlock user	Admin	20/10min
POST	/admin/users/{id}/roles	Assign roles	Admin	20/10min
DELETE	/admin/users/{id}/roles	Remove roles	Admin	20/10min
GET	/admin/registrations	List registrations	Admin	60/10min
GET	/admin/audit	System audit logs	Admin	60/10min
GET	/admin/metrics	System metrics	Admin	60/10min
GET	/admin/rate-limits	View rate limits	Admin	60/10min
📨 Invitations
Method	Endpoint	Description	Auth Required	Rate Limit
POST	/invitations	Create invitation	Admin	30/10min
GET	/invitations	List invitations	Admin	60/10min
GET	/invitations/{code}	Get invitation	No	30/10min
POST	/invitations/{code}/accept	Accept invitation	No	10/10min
DELETE	/invitations/{code}	Cancel invitation	Admin	20/10min
🛡️ Security
Method	Endpoint	Description	Auth Required	Rate Limit
GET	/security/mfa/setup	Setup MFA	Yes	10/10min
POST	/security/mfa/verify	Verify MFA	Yes	20/10min
DELETE	/security/mfa	Disable MFA	Yes	5/10min
GET	/security/devices	List trusted devices	Yes	30/10min
DELETE	/security/devices/{id}	Remove device	Yes	20/10min
📊 Analytics
Method	Endpoint	Description	Auth Required	Rate Limit
GET	/analytics/registrations	Registration stats	Admin	60/10min
GET	/analytics/logins	Login statistics	Admin	60/10min
GET	/analytics/active-users	Active user count	Admin	60/10min
🔧 System
Method	Endpoint	Description	Auth Required	Rate Limit
GET	/system/config	Public config	No	60/10min
POST	/system/webhooks/keycloak	Keycloak webhook	No	None
GET	/system/status	System status	Admin	30/10min
📋 Detailed Endpoint Specifications
1. POST /api/v1/register

Request:
json

{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePass123!",
  "confirm_password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1234567890",
  "source": "WEB",
  "metadata": {
    "registration_ip": "192.168.1.1",
    "user_agent": "Mozilla/5.0..."
  }
}

Response (201 Created):
json

{
  "registration_id": "uuid",
  "email": "user@example.com",
  "expires_at": "2024-01-01T12:00:00Z",
  "message": "Registration created. Check your email for verification."
}

2. POST /api/v1/verify

Request:
json

{
  "token": "verification-token-here",
  "metadata": {
    "verification_ip": "192.168.1.1",
    "user_agent": "Mozilla/5.0..."
  }
}

Response (200 OK):
json

{
  "user_id": "uuid",
  "email": "user@example.com",
  "message": "Account verified successfully",
  "access_token": "jwt-token",
  "refresh_token": "refresh-token",
  "expires_in": 3600
}

3. POST /api/v1/auth/login

Request:
json

{
  "email": "user@example.com",
  "password": "password123",
  "device_id": "mobile-123",
  "remember_me": true
}

Response (200 OK):
json

{
  "access_token": "jwt-token",
  "refresh_token": "refresh-token",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "first_name": "John",
    "last_name": "Doe",
    "roles": ["USER"],
    "email_verified": true
  }
}

4. GET /api/v1/users/me

Headers:
text

Authorization: Bearer <jwt-token>

Response (200 OK):
json

{
  "id": "uuid",
  "email": "user@example.com",
  "username": "johndoe",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1234567890",
  "roles": ["USER"],
  "status": "ACTIVE",
  "email_verified": true,
  "phone_verified": false,
  "created_at": "2024-01-01T12:00:00Z",
  "last_login_at": "2024-01-01T14:30:00Z",
  "metadata": {
    "failed_login_attempts": 0,
    "locked_until": null
  }
}

5. POST /api/v1/password/reset/request

Request:
json

{
  "email": "user@example.com",
  "redirect_url": "https://app.example.com/reset-password"
}

Response (200 OK):
json

{
  "message": "If an account exists, a reset email has been sent.",
  "expires_in": 1800
}

🔐 Authentication Requirements
Public Endpoints (No Auth):

    GET /health, /metrics, /version

    POST /register, /verify, /verify/resend

    POST /auth/login, /auth/refresh, /auth/validate

    POST /password/reset/request, /password/reset/confirm

    GET /password/policy, /system/config

User Endpoints (Requires Valid JWT):

    All /users/me/* endpoints

    POST /auth/logout

    POST /password/change

    All /security/* endpoints

Admin Endpoints (Requires Admin Role):

    All /admin/* endpoints

    All /analytics/* endpoints

    GET /system/status

⚡ Rate Limiting Tiers
Tier	Requests	Window	Applied To
Strict	5	10 min	Password reset, MFA setup
Standard	10	10 min	Registration, login
High	30	10 min	User operations
Very High	60	10 min	Admin operations
Unlimited	-	-	Health checks
📊 Response Status Codes
Code	Meaning	Typical Use
200	OK	Successful GET/PUT/PATCH
201	Created	Successful POST
204	No Content	Successful DELETE
400	Bad Request	Validation errors
401	Unauthorized	Missing/invalid token
403	Forbidden	Insufficient permissions
404	Not Found	Resource doesn't exist
409	Conflict	Resource already exists
422	Unprocessable Entity	Business rule violation
429	Too Many Requests	Rate limit exceeded
500	Internal Error	Server error
🗂️ Implementation Priority
Phase 1: MVP (Week 1-2)

    src/core/config.rs, errors.rs

    src/domain/entities.rs, traits.rs

    src/infrastructure/database/ (basic repos)

    src/application/services/user_registration_service.rs

    src/interfaces/http/handlers/health.rs, registration.rs

    POST /register, POST /verify, GET /health

Phase 2: Core Auth (Week 3-4)

    src/infrastructure/keycloak/

    src/application/services/auth_service.rs

    src/interfaces/http/handlers/auth.rs

    POST /auth/login, /logout, /refresh

    GET /users/me

Phase 3: User Management (Week 5-6)

    src/application/services/user_service.rs

    src/interfaces/http/handlers/user.rs, password.rs

    Profile endpoints, password management

    Audit logging

Phase 4: Admin & Security (Week 7-8)

    Admin endpoints

    Rate limiting

    MFA support

    Advanced security features

📝 Notes for Developers:
File Naming Conventions:

    Entities: user.rs, user_registration.rs

    Repositories: user_repository.rs, *_repository.rs

    Services: user_registration_service.rs

    Handlers: registration_handler.rs

    Middleware: auth_middleware.rs

Endpoint Grouping:

    Group related endpoints in same handler file

    Use consistent naming: /resource/{id}/action

    Version all endpoints: /api/v1/...

Testing Strategy:

    Unit tests: tests/unit/

    Integration: tests/integration/

    E2E: tests/e2e/

    Each handler should have corresponding test

Documentation:

    Each endpoint needs OpenAPI annotation

    Example requests/responses in docs

    Error responses documented








src/
├── main.rs
├── lib.rs
├── core/
│   ├── mod.rs
│   ├── config.rs           # AppConfig structs & loading
│   ├── errors.rs           # AppError enum & implementations
│   ├── constants.rs        # Application constants
│   ├── state.rs            # AppState struct
│   ├── logging.rs          # Tracing & logging setup
│   ├── metrics.rs          # Prometheus metrics
│   ├── security.rs         # Rate limiting, IP validation
│   └── validation.rs       # Custom validators
│
├── domain/
│   ├── mod.rs
│   ├── entities.rs         # User, UserRegistration, etc.
│   ├── value_objects.rs    # Email, Username, Password
│   ├── events.rs           # Domain events
│   ├── traits.rs           # Repository & service traits
│   └── enums.rs            # Status, Source, Role enums
│
├── application/
│   ├── mod.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── user_registration_service.rs
│   │   ├── user_service.rs
│   │   ├── auth_service.rs
│   │   ├── email_service.rs
│   │   └── audit_service.rs
│   ├── dto/
│   │   ├── mod.rs
│   │   ├── requests.rs     # Request DTOs
│   │   ├── responses.rs    # Response DTOs
│   │   └── commands.rs     # Command DTOs
│   └── queries.rs          # Query structs
│
├── infrastructure/
│   ├── mod.rs
│   ├── database/
│   │   ├── mod.rs
│   │   ├── connection.rs   # Database pool creation
│   │   ├── repositories/
│   │   │   ├── mod.rs
│   │   │   ├── user_repository.rs
│   │   │   ├── user_registration_repository.rs
│   │   │   ├── login_audit_repository.rs
│   │   │   ├── keycloak_sync_repository.rs
│   │   │   └── password_reset_repository.rs
│   │   └── migrations.rs   # Migration helpers
│   │
│   ├── keycloak/
│   │   ├── mod.rs
│   │   ├── client.rs       # KeycloakClient implementation
│   │   ├── models.rs       # Keycloak request/response models
│   │   └── auth.rs         # Token validation
│   │
│   ├── email/
│   │   ├── mod.rs
│   │   ├── smtp_client.rs
│   │   ├── templates.rs
│   │   └── mock_client.rs  # For testing
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── redis_client.rs
│   │   └── in_memory.rs    # For development
│   │
│   └── rate_limit/
│       ├── mod.rs
│       └── redis_store.rs
│
├── interfaces/
│   ├── mod.rs
│   └── http/
│       ├── mod.rs
│       ├── routes.rs       # Route definitions
│       ├── swagger.rs      # OpenAPI documentation
│       ├── middleware/
│       │   ├── mod.rs
│       │   ├── auth.rs
│       │   ├── rate_limit.rs
│       │   ├── logging.rs
│       │   └── cors.rs
│       ├── handlers/
│       │   ├── mod.rs
│       │   ├── health.rs
│       │   ├── registration.rs
│       │   ├── auth.rs
│       │   ├── user.rs
│       │   ├── admin.rs
│       │   ├── password.rs
│       │   └── invitations.rs
│       └── utils/
│           ├── mod.rs
│           ├── extractors.rs
│           └── responders.rs
│
├── jobs/
│   ├── mod.rs
│   ├── cleanup.rs          # Clean expired registrations
│   ├── sync.rs             # Sync with Keycloak
│   └── scheduler.rs        # Job scheduler
│
└── utils/
    ├── mod.rs
    ├── token.rs           # Token generation
    ├── crypto.rs          # Hashing utilities
    ├── time.rs           # Time utilities
    └── id_generator.rs   # ID generation







users - Core User Table
sql

CREATE TABLE users (
    -- Primary Identifiers
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    keycloak_id VARCHAR(255) UNIQUE NOT NULL,
    external_id VARCHAR(255), -- For external SSO providers
    
    -- Personal Information
    email VARCHAR(255) UNIQUE NOT NULL,
    email_normalized VARCHAR(255) GENERATED ALWAYS AS (LOWER(email)) STORED,
    username VARCHAR(100) UNIQUE,
    username_normalized VARCHAR(100) GENERATED ALWAYS AS (LOWER(username)) STORED,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    display_name VARCHAR(200) GENERATED ALWAYS AS (
        COALESCE(first_name || ' ' || last_name, username, email)
    ) STORED,
    phone VARCHAR(20),
    phone_normalized VARCHAR(20),
    avatar_url TEXT,
    date_of_birth DATE,
    gender VARCHAR(20),
    locale VARCHAR(10) DEFAULT 'en-US',
    timezone VARCHAR(50) DEFAULT 'UTC',
    
    -- Authentication & Security
    password_hash TEXT, -- Only for local accounts, NULL for SSO
    password_changed_at TIMESTAMPTZ,
    mfa_enabled BOOLEAN DEFAULT FALSE,
    mfa_method VARCHAR(20), -- 'TOTP', 'SMS', 'EMAIL', 'WEBAUTHN'
    mfa_secret TEXT,
    backup_codes TEXT[], -- Encrypted backup codes
    
    -- Roles & Permissions
    roles TEXT[] DEFAULT ARRAY['USER']::TEXT[],
    permissions TEXT[] DEFAULT ARRAY[]::TEXT[],
    scopes TEXT[] DEFAULT ARRAY[]::TEXT[],
    
    -- Status Flags
    status VARCHAR(50) DEFAULT 'PENDING' NOT NULL,
    status_reason TEXT,
    email_verified BOOLEAN DEFAULT FALSE,
    phone_verified BOOLEAN DEFAULT FALSE,
    account_verified BOOLEAN DEFAULT FALSE,
    requires_password_change BOOLEAN DEFAULT FALSE,
    
    -- Security & Lockout
    failed_login_attempts INTEGER DEFAULT 0,
    failed_password_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ,
    last_failed_login TIMESTAMPTZ,
    
    -- Source & Metadata
    source VARCHAR(50) DEFAULT 'WEB' NOT NULL,
    source_details JSONB DEFAULT '{}'::JSONB,
    registration_ip INET,
    registration_user_agent TEXT,
    
    -- Activity Tracking
    last_login_at TIMESTAMPTZ,
    last_login_ip INET,
    last_login_user_agent TEXT,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    login_count INTEGER DEFAULT 0,
    
    -- Privacy & Compliance
    accepted_terms_at TIMESTAMPTZ,
    accepted_privacy_at TIMESTAMPTZ,
    marketing_consent BOOLEAN DEFAULT FALSE,
    data_processing_consent BOOLEAN DEFAULT FALSE,
    gdpr_anonymized_at TIMESTAMPTZ,
    
    -- Audit & Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    version INTEGER DEFAULT 1 NOT NULL,
    
    -- Constraints
    CONSTRAINT users_status_check 
        CHECK (status IN ('PENDING', 'ACTIVE', 'INACTIVE', 'LOCKED', 'SUSPENDED', 'DELETED')),
    CONSTRAINT users_source_check 
        CHECK (source IN ('WEB', 'MOBILE', 'API', 'ADMIN', 'IMPORT', 'SSO')),
    CONSTRAINT users_mfa_method_check 
        CHECK (mfa_method IN ('TOTP', 'SMS', 'EMAIL', 'WEBAUTHN', NULL)),
    CONSTRAINT users_gender_check 
        CHECK (gender IN ('MALE', 'FEMALE', 'NON_BINARY', 'OTHER', 'PREFER_NOT_TO_SAY', NULL)),
    
    -- Soft delete constraint
    CONSTRAINT users_deleted_check 
        CHECK (deleted_at IS NULL OR status = 'DELETED')
);

2. user_registrations - Pending Registrations
sql

CREATE TABLE user_registrations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- User Details
    email VARCHAR(255) NOT NULL,
    email_normalized VARCHAR(255) GENERATED ALWAYS AS (LOWER(email)) STORED,
    username VARCHAR(100),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(20),
    
    -- Verification
    verification_token VARCHAR(255) UNIQUE NOT NULL,
    verification_token_hash VARCHAR(255) NOT NULL, -- For security
    verification_token_expires_at TIMESTAMPTZ NOT NULL,
    verification_method VARCHAR(50) DEFAULT 'EMAIL' NOT NULL, -- 'EMAIL', 'SMS', 'MANUAL'
    verified_at TIMESTAMPTZ,
    verification_ip INET,
    verification_user_agent TEXT,
    
    -- Temporary Password (for email verification flow)
    temp_password_hash TEXT,
    temp_password_expires_at TIMESTAMPTZ,
    
    -- Keycloak Integration
    keycloak_id VARCHAR(255),
    keycloak_sync_status VARCHAR(50) DEFAULT 'PENDING' NOT NULL,
    keycloak_sync_attempts INTEGER DEFAULT 0,
    keycloak_last_sync_at TIMESTAMPTZ,
    keycloak_sync_error TEXT,
    keycloak_sync_metadata JSONB DEFAULT '{}'::JSONB,
    
    -- Source & Context
    source VARCHAR(50) DEFAULT 'WEB' NOT NULL,
    registration_ip INET,
    user_agent TEXT,
    referrer TEXT,
    campaign_id VARCHAR(100),
    utm_source VARCHAR(100),
    utm_medium VARCHAR(100),
    utm_campaign VARCHAR(100),
    
    -- Status
    status VARCHAR(50) DEFAULT 'PENDING' NOT NULL,
    status_reason TEXT,
    
    -- Expiry
    expires_at TIMESTAMPTZ GENERATED ALWAYS AS (
        created_at + INTERVAL '24 hours'
    ) STORED,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT user_registrations_status_check 
        CHECK (status IN ('PENDING', 'VERIFIED', 'EXPIRED', 'CANCELLED', 'FAILED')),
    CONSTRAINT user_registrations_source_check 
        CHECK (source IN ('WEB', 'MOBILE', 'API', 'ADMIN', 'INVITATION')),
    CONSTRAINT user_registrations_verification_method_check 
        CHECK (verification_method IN ('EMAIL', 'SMS', 'MANUAL')),
    CONSTRAINT user_registrations_sync_status_check 
        CHECK (keycloak_sync_status IN ('PENDING', 'IN_PROGRESS', 'SUCCESS', 'FAILED', 'RETRY'))
);

3. login_audit_log - Authentication Audit Trail
sql

-- Main table (partitioned by month)
CREATE TABLE login_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- User Reference
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    email VARCHAR(255),
    username VARCHAR(100),
    
    -- Authentication Details
    attempt_type VARCHAR(50) NOT NULL, -- 'LOGIN', 'LOGOUT', 'REFRESH', 'VERIFICATION', 'MFA'
    auth_method VARCHAR(50) NOT NULL, -- 'PASSWORD', 'SSO', 'MAGIC_LINK', 'MFA'
    provider VARCHAR(50), -- 'keycloak', 'google', 'github', 'local'
    
    -- Result
    success BOOLEAN NOT NULL,
    status_code INTEGER,
    failure_reason TEXT,
    error_code VARCHAR(100),
    
    -- Security Context
    ip_address INET NOT NULL,
    ip_country_code CHAR(2),
    ip_region VARCHAR(100),
    ip_city VARCHAR(100),
    user_agent TEXT,
    device_id VARCHAR(255),
    device_type VARCHAR(50), -- 'DESKTOP', 'MOBILE', 'TABLET', 'UNKNOWN'
    device_name VARCHAR(255),
    browser VARCHAR(100),
    browser_version VARCHAR(50),
    os VARCHAR(100),
    os_version VARCHAR(50),
    
    -- Token Info (for successful auth)
    session_id VARCHAR(255),
    token_id VARCHAR(255),
    token_type VARCHAR(50), -- 'ACCESS', 'REFRESH', 'ID'
    token_expires_at TIMESTAMPTZ,
    
    -- MFA Context
    mfa_method VARCHAR(20),
    mfa_success BOOLEAN,
    
    -- Metadata
    risk_score INTEGER DEFAULT 0,
    risk_reasons TEXT[],
    flags TEXT[] DEFAULT ARRAY[]::TEXT[],
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT login_audit_attempt_type_check 
        CHECK (attempt_type IN ('LOGIN', 'LOGOUT', 'REFRESH', 'VERIFICATION', 'MFA', 'PASSWORD_RESET')),
    CONSTRAINT login_audit_auth_method_check 
        CHECK (auth_method IN ('PASSWORD', 'SSO', 'MAGIC_LINK', 'MFA', 'API_KEY', 'IMPERSONATION')),
    CONSTRAINT login_audit_device_type_check 
        CHECK (device_type IN ('DESKTOP', 'MOBILE', 'TABLET', 'UNKNOWN'))
) PARTITION BY RANGE (created_at);

4. keycloak_sync_log - Keycloak Synchronization Log
sql

CREATE TABLE keycloak_sync_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- References
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    registration_id UUID REFERENCES user_registrations(id) ON DELETE SET NULL,
    keycloak_user_id VARCHAR(255),
    
    -- Operation Details
    operation VARCHAR(50) NOT NULL, -- 'CREATE', 'UPDATE', 'DELETE', 'ENABLE', 'DISABLE'
    resource_type VARCHAR(50) NOT NULL, -- 'USER', 'ROLE', 'GROUP', 'CLIENT'
    resource_id VARCHAR(255),
    
    -- Request
    request_payload JSONB DEFAULT '{}'::JSONB,
    request_headers JSONB DEFAULT '{}'::JSONB,
    
    -- Response
    response_status INTEGER,
    response_body JSONB,
    response_headers JSONB,
    
    -- Sync Status
    sync_status VARCHAR(50) NOT NULL, -- 'PENDING', 'IN_PROGRESS', 'SUCCESS', 'FAILED', 'RETRY'
    retry_count INTEGER DEFAULT 0,
    next_retry_at TIMESTAMPTZ,
    
    -- Error Handling
    error_code VARCHAR(100),
    error_message TEXT,
    error_details JSONB,
    stack_trace TEXT,
    
    -- Metadata
    initiated_by VARCHAR(100), -- 'SYSTEM', 'USER', 'ADMIN'
    initiated_by_id UUID,
    correlation_id VARCHAR(255),
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    completed_at TIMESTAMPTZ,
    
    -- Constraints
    CONSTRAINT keycloak_sync_operation_check 
        CHECK (operation IN ('CREATE', 'UPDATE', 'DELETE', 'ENABLE', 'DISABLE', 'ASSIGN_ROLE', 'REMOVE_ROLE')),
    CONSTRAINT keycloak_sync_resource_type_check 
        CHECK (resource_type IN ('USER', 'ROLE', 'GROUP', 'CLIENT', 'REALM')),
    CONSTRAINT keycloak_sync_status_check 
        CHECK (sync_status IN ('PENDING', 'IN_PROGRESS', 'SUCCESS', 'FAILED', 'RETRY'))
);

5. password_reset_tokens - Password Reset Management
sql

CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    
    -- Token Details
    token VARCHAR(255) UNIQUE NOT NULL,
    token_hash VARCHAR(255) NOT NULL,
    token_type VARCHAR(50) DEFAULT 'PASSWORD_RESET' NOT NULL,
    
    -- Validity
    expires_at TIMESTAMPTZ NOT NULL,
    consumed_at TIMESTAMPTZ,
    invalidated_at TIMESTAMPTZ,
    
    -- Usage Context
    requested_by VARCHAR(50) DEFAULT 'USER' NOT NULL, -- 'USER', 'ADMIN', 'SYSTEM'
    requested_ip INET,
    requested_user_agent TEXT,
    
    -- Consumption Context
    consumed_ip INET,
    consumed_user_agent TEXT,
    new_password_hash TEXT, -- Store hash of new password for audit
    
    -- Security
    attempt_count INTEGER DEFAULT 0,
    last_attempt_at TIMESTAMPTZ,
    
    -- Metadata
    redirect_url TEXT,
    email_sent BOOLEAN DEFAULT FALSE,
    email_sent_at TIMESTAMPTZ,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT password_reset_tokens_type_check 
        CHECK (token_type IN ('PASSWORD_RESET', 'ACCOUNT_RECOVERY')),
    CONSTRAINT password_reset_tokens_requested_by_check 
        CHECK (requested_by IN ('USER', 'ADMIN', 'SYSTEM'))
);

6. email_verification_logs - Email Verification History
sql

CREATE TABLE email_verification_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- References
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    registration_id UUID REFERENCES user_registrations(id) ON DELETE SET NULL,
    email VARCHAR(255) NOT NULL,
    
    -- Verification Details
    verification_type VARCHAR(50) DEFAULT 'REGISTRATION' NOT NULL,
    token_used VARCHAR(255),
    verification_method VARCHAR(50) DEFAULT 'EMAIL' NOT NULL,
    
    -- Status
    status VARCHAR(50) NOT NULL,
    status_reason TEXT,
    
    -- Email Details
    email_template VARCHAR(100),
    email_subject TEXT,
    email_sent_at TIMESTAMPTZ,
    email_delivered_at TIMESTAMPTZ,
    email_opened_at TIMESTAMPTZ,
    email_clicked_at TIMESTAMPTZ,
    email_bounced BOOLEAN DEFAULT FALSE,
    bounce_reason TEXT,
    
    -- Context
    verification_ip INET,
    verification_user_agent TEXT,
    device_fingerprint VARCHAR(255),
    
    -- Metadata
    expires_at TIMESTAMPTZ,
    verified_from VARCHAR(50), -- 'EMAIL_LINK', 'MANUAL', 'API'
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT email_verification_type_check 
        CHECK (verification_type IN ('REGISTRATION', 'EMAIL_CHANGE', 'PASSWORD_RESET', 'ACCOUNT_RECOVERY')),
    CONSTRAINT email_verification_method_check 
        CHECK (verification_method IN ('EMAIL', 'SMS', 'MANUAL')),
    CONSTRAINT email_verification_status_check 
        CHECK (status IN ('SENT', 'DELIVERED', 'OPENED', 'CLICKED', 'VERIFIED', 'EXPIRED', 'BOUNCED', 'FAILED'))
);

7. rate_limits - Rate Limiting Storage
sql

CREATE TABLE rate_limits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Identifier
    bucket_key VARCHAR(255) NOT NULL,
    bucket_type VARCHAR(50) NOT NULL, -- 'IP', 'USER', 'GLOBAL', 'ENDPOINT'
    
    -- Rate Limit Counters
    request_count INTEGER DEFAULT 1 NOT NULL,
    window_start TIMESTAMPTZ NOT NULL,
    window_end TIMESTAMPTZ NOT NULL,
    
    -- Limits Configuration
    max_requests INTEGER NOT NULL,
    window_seconds INTEGER NOT NULL,
    
    -- Metadata
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    ip_address INET,
    endpoint VARCHAR(255),
    
    -- Status
    is_blocked BOOLEAN DEFAULT FALSE,
    blocked_until TIMESTAMPTZ,
    block_reason TEXT,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    last_request_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT rate_limits_bucket_type_check 
        CHECK (bucket_type IN ('IP', 'USER', 'GLOBAL', 'ENDPOINT')),
    CONSTRAINT rate_limits_window_check 
        CHECK (window_end > window_start)
);

-- Create a unique constraint for active buckets
CREATE UNIQUE INDEX rate_limits_active_bucket_idx 
ON rate_limits (bucket_key, bucket_type) 
WHERE NOT is_blocked AND window_end > NOW();

8. user_preferences - User Preferences & Settings
sql

CREATE TABLE user_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    
    -- Notification Preferences
    email_notifications BOOLEAN DEFAULT TRUE,
    push_notifications BOOLEAN DEFAULT TRUE,
    sms_notifications BOOLEAN DEFAULT FALSE,
    
    -- Email Types
    marketing_emails BOOLEAN DEFAULT FALSE,
    security_emails BOOLEAN DEFAULT TRUE,
    product_updates BOOLEAN DEFAULT TRUE,
    newsletter BOOLEAN DEFAULT FALSE,
    
    -- Security Preferences
    login_notifications BOOLEAN DEFAULT TRUE,
    password_change_notifications BOOLEAN DEFAULT TRUE,
    new_device_notifications BOOLEAN DEFAULT TRUE,
    
    -- Privacy Settings
    profile_visibility VARCHAR(50) DEFAULT 'PRIVATE', -- 'PUBLIC', 'PRIVATE', 'FRIENDS_ONLY'
    show_online_status BOOLEAN DEFAULT TRUE,
    show_last_seen BOOLEAN DEFAULT TRUE,
    data_sharing_consent BOOLEAN DEFAULT FALSE,
    
    -- Application Preferences
    theme VARCHAR(20) DEFAULT 'LIGHT', -- 'LIGHT', 'DARK', 'SYSTEM'
    language VARCHAR(10) DEFAULT 'en',
    time_format VARCHAR(10) DEFAULT '24H', -- '12H', '24H'
    date_format VARCHAR(20) DEFAULT 'YYYY-MM-DD',
    
    -- Custom Preferences (JSON store for app-specific settings)
    custom_preferences JSONB DEFAULT '{}'::JSONB,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    version INTEGER DEFAULT 1 NOT NULL,
    
    -- Constraints
    CONSTRAINT user_preferences_profile_visibility_check 
        CHECK (profile_visibility IN ('PUBLIC', 'PRIVATE', 'FRIENDS_ONLY')),
    CONSTRAINT user_preferences_theme_check 
        CHECK (theme IN ('LIGHT', 'DARK', 'SYSTEM')),
    CONSTRAINT user_preferences_time_format_check 
        CHECK (time_format IN ('12H', '24H')),
    
    -- One preference per user
    UNIQUE(user_id)
);

9. user_invitations - User Invitation System
sql

CREATE TABLE user_invitations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Invitation Details
    invitation_code VARCHAR(255) UNIQUE NOT NULL,
    invitation_token_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    
    -- Inviter Information
    invited_by_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    invited_by_email VARCHAR(255),
    invited_by_name VARCHAR(255),
    
    -- Invitee Information (if known)
    invited_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    invited_first_name VARCHAR(100),
    invited_last_name VARCHAR(100),
    
    -- Roles & Permissions
    assigned_roles TEXT[] DEFAULT ARRAY['USER']::TEXT[],
    assigned_permissions TEXT[] DEFAULT ARRAY[]::TEXT[],
    group_ids UUID[], -- Reference to groups if applicable
    
    -- Invitation Details
    invitation_type VARCHAR(50) DEFAULT 'REGULAR' NOT NULL,
    invitation_message TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    
    -- Status
    status VARCHAR(50) DEFAULT 'PENDING' NOT NULL,
    status_reason TEXT,
    
    -- Acceptance Details
    accepted_at TIMESTAMPTZ,
    accepted_ip INET,
    accepted_user_agent TEXT,
    registration_id UUID, -- Link to created registration
    
    -- Delivery
    sent_at TIMESTAMPTZ,
    sent_via VARCHAR(50), -- 'EMAIL', 'SMS', 'LINK'
    delivery_status VARCHAR(50),
    
    -- Metadata
    source VARCHAR(50) DEFAULT 'WEB',
    metadata JSONB DEFAULT '{}'::JSONB,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT user_invitations_type_check 
        CHECK (invitation_type IN ('REGULAR', 'ADMIN', 'BULK', 'AUTO')),
    CONSTRAINT user_invitations_status_check 
        CHECK (status IN ('PENDING', 'SENT', 'DELIVERED', 'ACCEPTED', 'EXPIRED', 'REVOKED', 'BOUNCED')),
    CONSTRAINT user_invitations_sent_via_check 
        CHECK (sent_via IN ('EMAIL', 'SMS', 'LINK', NULL))
);

10. trusted_devices - MFA & Trusted Devices
sql

CREATE TABLE trusted_devices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    
    -- Device Identification
    device_id VARCHAR(255) NOT NULL,
    device_name VARCHAR(255),
    device_type VARCHAR(50), -- 'DESKTOP', 'MOBILE', 'TABLET'
    device_os VARCHAR(100),
    device_os_version VARCHAR(50),
    browser VARCHAR(100),
    browser_version VARCHAR(50),
    
    -- Security
    public_key TEXT, -- For WebAuthn
    credential_id VARCHAR(512),
    user_handle VARCHAR(64),
    
    -- Trust & MFA
    is_trusted BOOLEAN DEFAULT FALSE,
    trusted_at TIMESTAMPTZ,
    trust_expires_at TIMESTAMPTZ,
    mfa_method VARCHAR(20), -- 'TOTP', 'WEBAUTHN', 'SMS'
    
    -- Location
    last_location VARCHAR(255),
    last_ip INET,
    last_country_code CHAR(2),
    
    -- Activity
    first_used_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    last_used_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    usage_count INTEGER DEFAULT 1,
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    revoked_at TIMESTAMPTZ,
    revocation_reason TEXT,
    
    -- Metadata
    user_agent TEXT,
    fingerprint VARCHAR(255),
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT trusted_devices_device_type_check 
        CHECK (device_type IN ('DESKTOP', 'MOBILE', 'TABLET', 'UNKNOWN')),
    CONSTRAINT trusted_devices_mfa_method_check 
        CHECK (mfa_method IN ('TOTP', 'WEBAUTHN', 'SMS', 'EMAIL', NULL)),
    
    -- Unique device per user
    UNIQUE(user_id, device_id)
);

11. webhook_events - External Webhook Events
sql

CREATE TABLE webhook_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Event Details
    event_type VARCHAR(100) NOT NULL,
    event_id VARCHAR(255) UNIQUE,
    payload JSONB NOT NULL,
    
    -- Source
    source_system VARCHAR(100) NOT NULL, -- 'KEYCLOAK', 'PAYMENT', 'NOTIFICATION'
    source_id VARCHAR(255),
    
    -- Processing Status
    status VARCHAR(50) DEFAULT 'PENDING' NOT NULL,
    processing_attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    next_attempt_at TIMESTAMPTZ,
    
    -- Error Handling
    error_message TEXT,
    error_stack_trace TEXT,
    last_attempt_at TIMESTAMPTZ,
    
    -- Delivery
    delivered_at TIMESTAMPTZ,
    response_status INTEGER,
    response_body TEXT,
    
    -- Metadata
    correlation_id VARCHAR(255),
    metadata JSONB DEFAULT '{}'::JSONB,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    
    -- Constraints
    CONSTRAINT webhook_events_status_check 
        CHECK (status IN ('PENDING', 'PROCESSING', 'DELIVERED', 'FAILED', 'RETRY'))
);

📈 Indexes for Performance
sql

-- Users table indexes
CREATE INDEX idx_users_email_normalized ON users(email_normalized);
CREATE INDEX idx_users_username_normalized ON users(username_normalized);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_last_activity_at ON users(last_activity_at);
CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NOT NULL;
CREATE INDEX idx_users_keycloak_id ON users(keycloak_id);
CREATE INDEX idx_users_external_id ON users(external_id) WHERE external_id IS NOT NULL;

-- User registrations indexes
CREATE INDEX idx_user_registrations_email_normalized ON user_registrations(email_normalized);
CREATE INDEX idx_user_registrations_verification_token_hash ON user_registrations(verification_token_hash);
CREATE INDEX idx_user_registrations_expires_at ON user_registrations(expires_at);
CREATE INDEX idx_user_registrations_status ON user_registrations(status);
CREATE INDEX idx_user_registrations_created_at ON user_registrations(created_at);
CREATE INDEX idx_user_registrations_keycloak_sync_status ON user_registrations(keycloak_sync_status);

-- Login audit log indexes (partition-aware)
CREATE INDEX idx_login_audit_user_id ON login_audit_log(user_id);
CREATE INDEX idx_login_audit_created_at ON login_audit_log(created_at);
CREATE INDEX idx_login_audit_ip_address ON login_audit_log(ip_address);
CREATE INDEX idx_login_audit_success ON login_audit_log(success);
CREATE INDEX idx_login_audit_session_id ON login_audit_log(session_id) WHERE session_id IS NOT NULL;

-- Keycloak sync log indexes
CREATE INDEX idx_keycloak_sync_user_id ON keycloak_sync_log(user_id);
CREATE INDEX idx_keycloak_sync_registration_id ON keycloak_sync_log(registration_id);
CREATE INDEX idx_keycloak_sync_status ON keycloak_sync_log(sync_status);
CREATE INDEX idx_keycloak_sync_created_at ON keycloak_sync_log(created_at);
CREATE INDEX idx_keycloak_sync_correlation_id ON keycloak_sync_log(correlation_id);

-- Password reset tokens indexes
CREATE INDEX idx_password_reset_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_token_hash ON password_reset_tokens(token_hash);
CREATE INDEX idx_password_reset_expires_at ON password_reset_tokens(expires_at);
CREATE INDEX idx_password_reset_consumed_at ON password_reset_tokens(consumed_at) WHERE consumed_at IS NULL;

-- Email verification logs indexes
CREATE INDEX idx_email_verification_user_id ON email_verification_logs(user_id);
CREATE INDEX idx_email_verification_email ON email_verification_logs(email);
CREATE INDEX idx_email_verification_status ON email_verification_logs(status);
CREATE INDEX idx_email_verification_created_at ON email_verification_logs(created_at);

-- Rate limits indexes
CREATE INDEX idx_rate_limits_bucket_key ON rate_limits(bucket_key);
CREATE INDEX idx_rate_limits_user_id ON rate_limits(user_id);
CREATE INDEX idx_rate_limits_ip_address ON rate_limits(ip_address);
CREATE INDEX idx_rate_limits_window_end ON rate_limits(window_end);
CREATE INDEX idx_rate_limits_is_blocked ON rate_limits(is_blocked);

-- User invitations indexes
CREATE INDEX idx_user_invitations_email ON user_invitations(email);
CREATE INDEX idx_user_invitations_invitation_code ON user_invitations(invitation_code);
CREATE INDEX idx_user_invitations_status ON user_invitations(status);
CREATE INDEX idx_user_invitations_expires_at ON user_invitations(expires_at);
CREATE INDEX idx_user_invitations_invited_by_user_id ON user_invitations(invited_by_user_id);

-- Trusted devices indexes
CREATE INDEX idx_trusted_devices_user_id ON trusted_devices(user_id);
CREATE INDEX idx_trusted_devices_device_id ON trusted_devices(device_id);
CREATE INDEX idx_trusted_devices_is_trusted ON trusted_devices(is_trusted);
CREATE INDEX idx_trusted_devices_last_used_at ON trusted_devices(last_used_at);

-- Webhook events indexes
CREATE INDEX idx_webhook_events_event_type ON webhook_events(event_type);
CREATE INDEX idx_webhook_events_status ON webhook_events(status);
CREATE INDEX idx_webhook_events_created_at ON webhook_events(created_at);
CREATE INDEX idx_webhook_events_next_attempt_at ON webhook_events(next_attempt_at);

🔧 Database Functions & Triggers
sql

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to generate normalized fields
CREATE OR REPLACE FUNCTION normalize_phone(phone TEXT)
RETURNS TEXT AS $$
BEGIN
    RETURN REGEXP_REPLACE(phone, '[^0-9]', '', 'g');
END;
$$ LANGUAGE plpgsql;

-- Function to check if user can login
CREATE OR REPLACE FUNCTION can_user_login(user_id UUID)
RETURNS TABLE(can_login BOOLEAN, reason TEXT) AS $$
DECLARE
    user_record RECORD;
BEGIN
    SELECT status, locked_until, failed_login_attempts
    INTO user_record
    FROM users
    WHERE id = user_id;
    
    IF NOT FOUND THEN
        RETURN QUERY SELECT FALSE, 'User not found';
        RETURN;
    END IF;
    
    IF user_record.status != 'ACTIVE' THEN
        RETURN QUERY SELECT FALSE, 'Account is ' || user_record.status;
        RETURN;
    END IF;
    
    IF user_record.locked_until IS NOT NULL AND user_record.locked_until > NOW() THEN
        RETURN QUERY SELECT FALSE, 'Account locked until ' || user_record.locked_until;
        RETURN;
    END IF;
    
    RETURN QUERY SELECT TRUE, NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to clean up expired data
CREATE OR REPLACE FUNCTION cleanup_expired_data()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER := 0;
BEGIN
    -- Clean expired registrations (older than 7 days)
    WITH deleted AS (
        DELETE FROM user_registrations 
        WHERE expires_at < NOW() - INTERVAL '7 days'
        RETURNING id
    )
    SELECT COUNT(*) INTO deleted_count FROM deleted;
    
    -- Clean expired password reset tokens
    DELETE FROM password_reset_tokens 
    WHERE expires_at < NOW() - INTERVAL '7 days';
    
    -- Clean expired rate limits (older than 30 days)
    DELETE FROM rate_limits 
    WHERE window_end < NOW() - INTERVAL '30 days';
    
    -- Clean old webhook events (older than 90 days)
    DELETE FROM webhook_events 
    WHERE created_at < NOW() - INTERVAL '90 days';
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Triggers for updated_at
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_registrations_updated_at
    BEFORE UPDATE ON user_registrations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_keycloak_sync_log_updated_at
    BEFORE UPDATE ON keycloak_sync_log
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger to normalize phone numbers
CREATE OR REPLACE FUNCTION normalize_user_phone()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.phone IS NOT NULL THEN
        NEW.phone_normalized = normalize_phone(NEW.phone);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER normalize_users_phone
    BEFORE INSERT OR UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION normalize_user_phone();

📊 Table Partitions
sql

-- Create monthly partitions for login_audit_log
CREATE TABLE login_audit_log_2024_01 PARTITION OF login_audit_log
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

CREATE TABLE login_audit_log_2024_02 PARTITION OF login_audit_log
    FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');

-- Create monthly partitions for keycloak_sync_log
CREATE TABLE keycloak_sync_log_2024_01 PARTITION OF keycloak_sync_log
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

-- Create monthly partitions for webhook_events
CREATE TABLE webhook_events_2024_01 PARTITION OF webhook_events
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

-- Function to create new partitions automatically
CREATE OR REPLACE FUNCTION create_monthly_partition()
RETURNS VOID AS $$
DECLARE
    next_month TEXT;
    partition_name TEXT;
BEGIN
    next_month := TO_CHAR(NOW() + INTERVAL '1 month', 'YYYY_MM');
    
    -- Create login_audit_log partition
    partition_name := 'login_audit_log_' || next_month;
    EXECUTE format(
        'CREATE TABLE IF NOT EXISTS %I PARTITION OF login_audit_log
        FOR VALUES FROM (%L) TO (%L)',
        partition_name,
        DATE_TRUNC('month', NOW() + INTERVAL '1 month'),
        DATE_TRUNC('month', NOW() + INTERVAL '2 months')
    );
    
    -- Create keycloak_sync_log partition
    partition_name := 'keycloak_sync_log_' || next_month;
    EXECUTE format(
        'CREATE TABLE IF NOT EXISTS %I PARTITION OF keycloak_sync_log
        FOR VALUES FROM (%L) TO (%L)',
        partition_name,
        DATE_TRUNC('month', NOW() + INTERVAL '1 month'),
        DATE_TRUNC('month', NOW() + INTERVAL '2 months')
    );
END;
$$ LANGUAGE plpgsql;

🔐 Security Views & Row Level Security
sql

-- View for active users (excludes deleted/suspended)
CREATE VIEW active_users AS
SELECT id, email, username, first_name, last_name, status, created_at
FROM users
WHERE status = 'ACTIVE'

This response is AI-generated, for reference only.













🔄 Complete Registration Workflow
📊 Workflow Overview
📋 Detailed Step-by-Step Flow
Phase 1: Registration Request
Step 1: User Submits Registration Form
json

POST /api/v1/register
{
  "email": "john.doe@example.com",
  "username": "johndoe",
  "password": "SecurePass123!",
  "confirm_password": "SecurePass123!",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1234567890",
  "source": "WEB",
  "metadata": {
    "registration_ip": "192.168.1.100",
    "user_agent": "Mozilla/5.0...",
    "accept_language": "en-US",
    "timezone": "America/New_York"
  }
}

Step 2: Validation & Sanitization
rust

// Inside UserRegistrationService::register_user()
1. Validate email format & uniqueness
2. Validate password strength:
   - Minimum 8 characters
   - Contains uppercase, lowercase, number, special char
   - Not in common passwords list
3. Check password == confirm_password
4. Sanitize inputs (trim, lowercase email, etc.)
5. Validate phone number format (if provided)
6. Validate source (WEB, MOBILE, API, ADMIN)

Step 3: Check for Existing Users
sql

-- Check active users
SELECT * FROM users 
WHERE email_normalized = LOWER('john.doe@example.com') 
  AND deleted_at IS NULL;

-- Check pending registrations
SELECT * FROM user_registrations 
WHERE email_normalized = LOWER('john.doe@example.com') 
  AND status = 'PENDING' 
  AND expires_at > NOW();

Step 4: Generate Verification Token
rust

// Secure token generation
let verification_token = use nanoid 16
let token_hash = argon2::hash_encoded(
    verification_token.as_bytes(),
    &generate_salt(),
    &argon2::Config::default()
)?;
let expires_at = Utc::now() + Duration::hours(24);

Step 5: Create Registration Record
sql

registration_id = REG-nanodid(16)

INSERT INTO user_registrations (
    id, email, email_normalized, username, 
    first_name, last_name, phone,
    verification_token, verification_token_hash,
    verification_token_expires_at,
    source, registration_ip, user_agent,
    status, keycloak_sync_status,
    created_at, updated_at
) VALUES (
    'uuid', 'john.doe@example.com', 'john.doe@example.com',
    'johndoe', 'John', 'Doe', '+1234567890',
    'raw-token', 'hashed-token',
    '2024-01-01T12:00:00Z',
    'WEB', '192.168.1.100', 'Mozilla/5.0...',
    'PENDING', 'PENDING',
    NOW(), NOW()
);

Step 6: Create Disabled User in Keycloak
rust

// Keycloak API call
POST /admin/realms/{realm}/users
{
  "username": "johndoe",
  "email": "john.doe@example.com",
  "firstName": "John",
  "lastName": "Doe",
  "enabled": false,  // IMPORTANT: Disabled until verified
  "emailVerified": false,
  "credentials": [{
    "type": "password",
    "value": "SecurePass123!",
    "temporary": false
  }],
  "attributes": {
    "registration_id": "uuid-from-db",
    "source": "WEB",
    "phone": "+1234567890"
  }
}

Step 7: Store Keycloak ID & Update Sync Status
sql

UPDATE user_registrations 
SET keycloak_id = 'keycloak-uuid',
    keycloak_sync_status = 'SUCCESS',
    keycloak_last_sync_at = NOW(),
    updated_at = NOW()
WHERE id = 'registration-uuid';

Step 8: Send Verification Email
rust

// Email template
Subject: Verify Your Email Address - Welcome to Our Service!

Body:
Hello John,

Please verify your email address by clicking the link below:
https://app.example.com/verify-email?token=verification-token

This link will expire in 24 hours.

If you didn't create an account, please ignore this email.

Best regards,
The Team

Step 9: Return Response
json

HTTP/1.1 201 Created
{
  "registration_id": "uuid",
  "email": "john.doe@example.com",
  "expires_at": "2024-01-01T12:00:00Z",
  "message": "Registration created. Please check your email for verification instructions.",
  "verification_method": "EMAIL",
  "resend_available_in": 300  // Can resend in 5 minutes
}

Phase 2: Email Verification
Step 10: User Clicks Verification Link
text

GET /api/v1/verify?token=verification-token
# OR via frontend:
POST /api/v1/verify
{
  "token": "verification-token",
  "metadata": {
    "verification_ip": "192.168.1.100",
    "user_agent": "Mozilla/5.0...",
    "device_fingerprint": "abc123"
  }
}

Step 11: Token Validation
sql

-- Find registration by token hash
SELECT * FROM user_registrations 
WHERE verification_token_hash = $1 
  AND status = 'PENDING'
  AND verification_token_expires_at > NOW();

Validation Checks:

    Token exists and is not expired

    Registration status is PENDING

    Email not already verified

    Keycloak user exists and is disabled

    Token not already used

Step 12: Enable User in Keycloak
rust

// Enable the user in Keycloak
PUT /admin/realms/{realm}/users/{keycloak-id}
{
  "enabled": true
}

// Optionally verify email in Keycloak
{
  "emailVerified": true
}

Step 13: Create User Record in Local Database
sql

-- Start transaction
BEGIN;

-- Mark registration as verified
UPDATE user_registrations 
SET status = 'VERIFIED',
    verified_at = NOW(),
    verification_ip = '192.168.1.100',
    verification_user_agent = 'Mozilla/5.0...',
    updated_at = NOW()
WHERE id = 'registration-uuid';

-- Create user record
INSERT INTO users (
    id, keycloak_id, email, email_normalized,
    username, username_normalized,
    first_name, last_name, phone,
    roles, status, email_verified,
    source, registration_ip, registration_user_agent,
    created_at, updated_at
) VALUES (
    'user-uuid', 'keycloak-uuid', 
    'john.doe@example.com', 'john.doe@example.com',
    'johndoe', 'johndoe',
    'John', 'Doe', '+1234567890',
    ARRAY['USER'], 'ACTIVE', true,
    'WEB', '192.168.1.100', 'Mozilla/5.0...',
    NOW(), NOW()
);

-- Create default user preferences
INSERT INTO user_preferences (user_id) VALUES ('user-uuid');

-- Create audit log
INSERT INTO login_audit_log (
    user_id, email, attempt_type, auth_method,
    success, ip_address, user_agent
) VALUES (
    'user-uuid', 'john.doe@example.com',
    'VERIFICATION', 'EMAIL', true,
    '192.168.1.100', 'Mozilla/5.0...'
);

COMMIT;

Step 14: Generate Authentication Tokens
rust

// Get tokens from Keycloak
POST /realms/{realm}/protocol/openid-connect/token
grant_type=password&
username=johndoe&
password=SecurePass123!&
client_id=auth-client&
client_secret=client-secret

// Response
{
  "access_token": "jwt-token",
  "refresh_token": "refresh-token",
  "expires_in": 3600,
  "token_type": "Bearer"
}

Step 15: Send Welcome Email
rust

// Welcome email template
Subject: Welcome to Our Service, John!

Body:
Hello John,

Your account has been successfully verified!

Account Details:
- Email: john.doe@example.com
- Username: johndoe
- Account created: January 1, 2024

Get started by:
1. Complete your profile
2. Explore our features
3. Set up two-factor authentication

Need help? Contact support@example.com

Best regards,
The Team

Step 16: Return Success Response
json

HTTP/1.1 200 OK
{
  "user_id": "user-uuid",
  "email": "john.doe@example.com",
  "username": "johndoe",
  "message": "Account verified successfully",
  "access_token": "jwt-token",
  "refresh_token": "refresh-token",
  "expires_in": 3600,
  "token_type": "Bearer",
  "user": {
    "id": "user-uuid",
    "email": "john.doe@example.com",
    "username": "johndoe",
    "first_name": "John",
    "last_name": "Doe",
    "roles": ["USER"],
    "email_verified": true,
    "created_at": "2024-01-01T12:00:00Z"
  }
}

🔄 Alternative Verification Methods
SMS Verification Flow
rust

// Step 1: During registration
if command.phone.is_some() {
    let sms_code = generate_6_digit_code();
    send_sms(command.phone.unwrap(), sms_code);
    store_sms_verification(registration_id, sms_code);
    verification_method = "SMS";
}

// Step 2: Verification endpoint
POST /api/v1/verify/sms
{
  "registration_id": "uuid",
  "code": "123456",
  "phone": "+1234567890"
}

Manual Verification (Admin)
rust

// Admin triggers verification
POST /api/v1/admin/registrations/{id}/verify
Headers: Authorization: Bearer admin-token

// Admin can also reject
POST /api/v1/admin/registrations/{id}/reject
{
  "reason": "Suspicious activity detected"
}

⚡ Error Handling Scenarios
Scenario 1: Token Expired
json

HTTP/1.1 410 Gone
{
  "error": "token_expired",
  "message": "Verification token has expired",
  "can_resend": true,
  "resend_endpoint": "/api/v1/verify/resend"
}

Scenario 2: Invalid Token
json

HTTP/1.1 400 Bad Request
{
  "error": "invalid_token",
  "message": "Invalid verification token",
  "hint": "Check the token or request a new one"
}

Scenario 3: Keycloak Failure
rust

// Fallback strategy
1. Mark registration as "KEYCLOAK_FAILED"
2. Store error in keycloak_sync_log
3. Implement retry mechanism
4. Alert administrators
5. Allow manual intervention

Scenario 4: Email Delivery Failure
rust

// Retry strategy
1. Mark email_sent = false
2. Implement exponential backoff for retries
3. After 3 failures, mark as "EMAIL_FAILED"
4. Provide alternative verification methods

🔒 Security Considerations
Token Security
rust

// Best practices
1. Use cryptographically secure random tokens (UUID v4)
2. Store only hashed tokens in database
3. Set appropriate expiration (24 hours for registration)
4. One-time use tokens
5. Rate limit token validation attempts

Prevention Measures
rust

1. Rate limiting per IP/email for registration
2. Email domain validation/blocklist
3. Password strength validation
4. CAPTCHA for suspicious patterns
5. IP reputation checks

Data Privacy
rust

1. Don't expose if email exists in error messages
2. Use generic messages: "If an account exists..."
3. GDPR compliance - store consent
4. Data minimization - only collect necessary fields

📊 Registration States & Transitions
🛠️ Database Tables Involved
Table	Purpose	Key Fields
user_registrations	Store pending registrations	verification_token, expires_at, status
users	Store verified users	keycloak_id, email, status
keycloak_sync_log	Track Keycloak operations	operation, sync_status, error_message
email_verification_logs	Track verification attempts	verification_type, status, email_sent_at
login_audit_log	Audit trail	attempt_type, success, ip_address
⚙️ Configuration Parameters
rust

// config.yml
registration:
  verification_token_expiry_hours: 24
  resend_cooldown_seconds: 300
  max_pending_registrations_per_email: 3
  max_pending_registrations_per_ip: 10
  allow_disposable_emails: false
  required_fields: ["email", "password"]
  optional_fields: ["username", "first_name", "last_name", "phone"]
  
keycloak:
  create_user_on_registration: true
  enable_user_on_verification: true
  assign_default_roles: ["USER"]
  require_email_verification: true
  
email:
  verification_template: "email-verification"
  welcome_template: "welcome-email"
  from_address: "noreply@example.com"
  reply_to: "support@example.com"

🔍 Monitoring & Metrics
rust

// Metrics to track
metrics::counter!("registrations_total", "type" => "attempt");
metrics::counter!("registrations_success", "type" => "success");
metrics::counter!("registrations_failed", "type" => "validation_error");
metrics::counter!("verifications_total", "type" => "attempt");
metrics::counter!("verifications_success", "type" => "success");
metrics::counter!("verifications_failed", "type" => "expired");
metrics::gauge!("pending_registrations", count);
metrics::histogram!("registration_duration_seconds", duration);

🧪 Testing Strategy
Unit Tests
rust

#[cfg(test)]
mod tests {
    // Test validation logic
    #[test]
    fn test_email_validation() { /* ... */ }
    
    // Test token generation
    #[test]
    fn test_token_expiry() { /* ... */ }
    
    // Test password strength
    #[test]
    fn test_password_validation() { /* ... */ }
}

Integration Tests
rust

// Test full registration flow
#[tokio::test]
async fn test_registration_verification_flow() {
    // 1. Register user
    // 2. Verify email
    // 3. Check user created
    // 4. Check Keycloak user enabled
}

E2E Tests
rust

// Test with real Keycloak instance
#[test]
fn test_registration_with_keycloak() {
    // Use testcontainers for Keycloak
    // Test complete flow
}

🚀 Optimizations
Performance
rust

1. Use database connection pooling
2. Implement caching for rate limits
3. Async email sending
4. Batch database operations
5. Use database transactions

Scalability
rust

1. Horizontal scaling with shared database
2. Redis for rate limiting and caching
3. Message queue for email sending
4. Database read replicas
5. CDN for static assets (verification pages)

