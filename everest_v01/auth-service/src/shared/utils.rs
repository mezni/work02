use uuid::Uuid;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use argon2::{self, Config};
use chrono::{DateTime, Utc};
use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::shared::constants;

// Password utilities
pub struct PasswordUtils;

impl PasswordUtils {
    pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
        let salt: [u8; 32] = thread_rng().gen();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), &salt, &config)
    }
    
    pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(hash, password.as_bytes())
    }
    
    pub fn generate_strong_password() -> String {
        let mut rng = thread_rng();
        
        // Generate password with requirements
        let mut password = String::new();
        
        // Add uppercase letter
        password.push(rng.gen_range('A'..='Z'));
        
        // Add lowercase letter
        password.push(rng.gen_range('a'..='z'));
        
        // Add digit
        password.push(rng.gen_range('0'..='9'));
        
        // Add special character
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        password.push(
            special_chars
                .chars()
                .nth(rng.gen_range(0..special_chars.len()))
                .unwrap(),
        );
        
        // Add remaining random characters
        for _ in 0..8 {
            password.push(rng.sample(Alphanumeric) as char);
        }
        
        // Shuffle the password
        let mut chars: Vec<char> = password.chars().collect();
        rng.shuffle(&mut chars);
        chars.into_iter().collect()
    }
    
    pub fn is_password_strong(password: &str) -> bool {
        if password.len() < constants::MIN_PASSWORD_LENGTH {
            return false;
        }
        
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        
        has_uppercase && has_lowercase && has_digit && has_special
    }
}

// Token utilities
pub struct TokenUtils;

impl TokenUtils {
    pub fn generate_random_token(length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
    
    pub fn generate_uuid_token() -> String {
        Uuid::new_v4().to_string()
    }
    
    pub fn generate_secure_token() -> String {
        format!(
            "{}-{}",
            Self::generate_uuid_token(),
            Self::generate_random_token(16)
        )
    }
    
    pub fn is_token_expired(expires_at: &DateTime<Utc>) -> bool {
        Utc::now() > *expires_at
    }
    
    pub fn time_until_expiry(expires_at: &DateTime<Utc>) -> i64 {
        let now = Utc::now();
        if now > *expires_at {
            0
        } else {
            (*expires_at - now).num_seconds()
        }
    }
}

// Validation utilities
pub struct ValidationUtils;

impl ValidationUtils {
    pub fn is_valid_email(email: &str) -> bool {
        if email.len() > constants::MAX_EMAIL_LENGTH {
            return false;
        }
        
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        ).unwrap();
        
        email_regex.is_match(email)
    }
    
    pub fn sanitize_input(input: &str) -> String {
        input.trim().to_string()
    }
    
    pub fn is_valid_company_name(name: &str) -> bool {
        let trimmed = name.trim();
        !trimmed.is_empty() && trimmed.len() <= constants::MAX_COMPANY_NAME_LENGTH
    }
    
    pub fn is_valid_station_name(name: &str) -> bool {
        let trimmed = name.trim();
        !trimmed.is_empty() && trimmed.len() <= constants::MAX_STATION_NAME_LENGTH
    }
    
    pub fn validate_uuid(uuid_str: &str) -> Result<Uuid, String> {
        Uuid::parse_str(uuid_str)
            .map_err(|e| format!("Invalid UUID: {}", e))
    }
}

// Date/Time utilities
pub struct DateTimeUtils;

impl DateTimeUtils {
    pub fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
    
    pub fn format_date_time(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }
    
    pub fn parse_date_time(dt_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(dt_str)
            .map(|dt| dt.with_timezone(&Utc))
    }
    
    pub fn add_days(dt: &DateTime<Utc>, days: i64) -> DateTime<Utc> {
        *dt + chrono::Duration::days(days)
    }
    
    pub fn add_hours(dt: &DateTime<Utc>, hours: i64) -> DateTime<Utc> {
        *dt + chrono::Duration::hours(hours)
    }
    
    pub fn add_minutes(dt: &DateTime<Utc>, minutes: i64) -> DateTime<Utc> {
        *dt + chrono::Duration::minutes(minutes)
    }
}

// HTTP utilities
pub struct HttpUtils;

impl HttpUtils {
    pub fn extract_bearer_token(auth_header: &str) -> Option<String> {
        if auth_header.starts_with(constants::TOKEN_PREFIX) {
            Some(auth_header[constants::TOKEN_PREFIX.len()..].trim().to_string())
        } else {
            None
        }
    }
    
    pub fn get_client_ip(req: &actix_web::HttpRequest) -> Option<String> {
        req.connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string())
    }
    
    pub fn get_user_agent(req: &actix_web::HttpRequest) -> Option<String> {
        req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
    }
}

// Rate limiting utilities
pub struct RateLimitUtils;

impl RateLimitUtils {
    pub fn generate_rate_limit_key(identifier: &str, endpoint: &str) -> String {
        format!("rate_limit:{}:{}", identifier, endpoint)
    }
    
    pub fn generate_ip_rate_limit_key(ip: &str, endpoint: &str) -> String {
        Self::generate_rate_limit_key(ip, endpoint)
    }
    
    pub fn generate_user_rate_limit_key(user_id: &Uuid, endpoint: &str) -> String {
        Self::generate_rate_limit_key(&user_id.to_string(), endpoint)
    }
    
    pub fn is_rate_limited(
        current_count: u32,
        limit: u32,
        window_start: i64,
        window_seconds: u64,
    ) -> bool {
        let now = DateTimeUtils::current_timestamp();
        let window_end = window_start + window_seconds as i64;
        
        if now > window_end {
            // Window has expired, reset count
            false
        } else {
            current_count >= limit
        }
    }
}

// Cache key utilities
pub struct CacheKeyUtils;

impl CacheKeyUtils {
    pub fn user_key(user_id: &Uuid) -> String {
        format!("user:{}", user_id)
    }
    
    pub fn user_email_key(email: &str) -> String {
        format!("user:email:{}", email)
    }
    
    pub fn token_key(token: &str) -> String {
        format!("token:{}", token)
    }
    
    pub fn session_key(user_id: &Uuid) -> String {
        format!("session:{}", user_id)
    }
    
    pub fn verification_token_key(token: &str) -> String {
        format!("verification:{}", token)
    }
    
    pub fn password_reset_token_key(token: &str) -> String {
        format!("password_reset:{}", token)
    }
}

// Serialization utilities
pub struct SerializationUtils;

impl SerializationUtils {
    pub fn serialize_to_json<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }
    
    pub fn serialize_to_json_pretty<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(value)
    }
    
    pub fn deserialize_from_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    pub fn to_base64<T: AsRef<[u8]>>(data: T) -> String {
        base64::encode(data)
    }
    
    pub fn from_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(encoded)
    }
}

// Environment utilities
pub struct EnvUtils;

impl EnvUtils {
    pub fn get_env_var(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
    
    pub fn get_env_var_or_default(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }
    
    pub fn is_production() -> bool {
        Self::get_env_var_or_default("APP_ENVIRONMENT", "development") == "production"
    }
    
    pub fn is_development() -> bool {
        Self::get_env_var_or_default("APP_ENVIRONMENT", "development") == "development"
    }
    
    pub fn is_test() -> bool {
        Self::get_env_var_or_default("APP_ENVIRONMENT", "development") == "test"
    }
}