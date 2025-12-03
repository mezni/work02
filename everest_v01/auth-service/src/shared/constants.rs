// Authentication constants
pub const TOKEN_HEADER: &str = "Authorization";
pub const TOKEN_PREFIX: &str = "Bearer ";
pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
pub const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";

// Token expiration constants (in seconds)
pub const ACCESS_TOKEN_EXPIRATION: i64 = 24 * 60 * 60; // 24 hours
pub const REFRESH_TOKEN_EXPIRATION: i64 = 30 * 24 * 60 * 60; // 30 days
pub const EMAIL_VERIFICATION_TOKEN_EXPIRATION: i64 = 24 * 60 * 60; // 24 hours
pub const PASSWORD_RESET_TOKEN_EXPIRATION: i64 = 1 * 60 * 60; // 1 hour

// Rate limiting constants
pub const RATE_LIMIT_WINDOW: u64 = 60; // 1 minute in seconds
pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
pub const MAX_REGISTRATION_ATTEMPTS_PER_HOUR: u32 = 3;
pub const MAX_PASSWORD_RESET_ATTEMPTS_PER_HOUR: u32 = 3;

// Password requirements
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;

// Validation constants
pub const MAX_EMAIL_LENGTH: usize = 254;
pub const MAX_COMPANY_NAME_LENGTH: usize = 100;
pub const MAX_STATION_NAME_LENGTH: usize = 50;

// Cache TTLs (in seconds)
pub const USER_CACHE_TTL: u64 = 5 * 60; // 5 minutes
pub const TOKEN_CACHE_TTL: u64 = 1 * 60; // 1 minute
pub const RATE_LIMIT_CACHE_TTL: u64 = 60; // 1 minute

// API constants
pub const API_VERSION: &str = "v1";
pub const API_PREFIX: &str = "/api/v1";
pub const HEALTH_CHECK_PATH: &str = "/health";
pub const SWAGGER_UI_PATH: &str = "/swagger-ui/";

// Error codes
pub mod error_codes {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const AUTHENTICATION_ERROR: &str = "AUTHENTICATION_ERROR";
    pub const AUTHORIZATION_ERROR: &str = "AUTHORIZATION_ERROR";
    pub const NOT_FOUND_ERROR: &str = "NOT_FOUND_ERROR";
    pub const CONFLICT_ERROR: &str = "CONFLICT_ERROR";
    pub const RATE_LIMIT_ERROR: &str = "RATE_LIMIT_ERROR";
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    pub const SERVICE_UNAVAILABLE: &str = "SERVICE_UNAVAILABLE";
    pub const TOKEN_EXPIRED: &str = "TOKEN_EXPIRED";
    pub const TOKEN_INVALID: &str = "TOKEN_INVALID";
    pub const USER_DISABLED: &str = "USER_DISABLED";
    pub const EMAIL_NOT_VERIFIED: &str = "EMAIL_NOT_VERIFIED";
    pub const WEAK_PASSWORD: &str = "WEAK_PASSWORD";
    pub const EMAIL_ALREADY_EXISTS: &str = "EMAIL_ALREADY_EXISTS";
}

// Keycloak constants
pub mod keycloak {
    pub const GRANT_TYPE_PASSWORD: &str = "password";
    pub const GRANT_TYPE_REFRESH_TOKEN: &str = "refresh_token";
    pub const GRANT_TYPE_CLIENT_CREDENTIALS: &str = "client_credentials";
    pub const TOKEN_TYPE_BEARER: &str = "Bearer";
    
    // Default realm roles
    pub const ROLE_USER: &str = "user";
    pub const ROLE_ADMIN: &str = "admin";
    pub const ROLE_MANAGER: &str = "manager";
    
    // User attributes
    pub const ATTR_COMPANY_NAME: &str = "company_name";
    pub const ATTR_STATION_NAME: &str = "station_name";
    pub const ATTR_CREATED_AT: &str = "created_at";
    pub const ATTR_UPDATED_AT: &str = "updated_at";
}

// Logging constants
pub mod logging {
    pub const LOG_FORMAT_JSON: &str = "json";
    pub const LOG_FORMAT_TEXT: &str = "text";
    
    // Log field names
    pub const FIELD_USER_ID: &str = "user_id";
    pub const FIELD_EMAIL: &str = "email";
    pub const FIELD_IP_ADDRESS: &str = "ip_address";
    pub const FIELD_USER_AGENT: &str = "user_agent";
    pub const FIELD_REQUEST_ID: &str = "request_id";
    pub const FIELD_DURATION_MS: &str = "duration_ms";
    pub const FIELD_STATUS_CODE: &str = "status_code";
    pub const FIELD_ERROR_TYPE: &str = "error_type";
}

// Environment constants
pub mod environment {
    pub const DEVELOPMENT: &str = "development";
    pub const STAGING: &str = "staging";
    pub const PRODUCTION: &str = "production";
    pub const TEST: &str = "test";
}