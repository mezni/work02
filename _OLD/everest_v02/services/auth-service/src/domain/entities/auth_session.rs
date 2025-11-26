use crate::domain::value_objects::UserId;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

/// Authentication Session Entity
#[derive(Debug, Clone)]
pub struct AuthSession {
    // Identity
    session_id: String,
    user_id: UserId,

    // State
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
    revoked: bool,
    device_info: String,
    ip_address: String,

    // Metadata
    created_at: DateTime<Utc>,
    last_used_at: DateTime<Utc>,
}

impl AuthSession {
    /// Create a new authentication session
    pub fn new(
        user_id: UserId,
        device_info: String,
        ip_address: String,
        token_duration: Duration,
    ) -> Self {
        let now = Utc::now();
        Self {
            session_id: format!("sess_{}", Uuid::new_v4()),
            user_id,
            access_token: format!("access_{}", Uuid::new_v4()),
            refresh_token: format!("refresh_{}", Uuid::new_v4()),
            expires_at: now + token_duration,
            revoked: false,
            device_info,
            ip_address,
            created_at: now,
            last_used_at: now,
        }
    }

    // Identity access
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    // Token access
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    // State access
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn revoked(&self) -> bool {
        self.revoked
    }

    pub fn device_info(&self) -> &str {
        &self.device_info
    }

    pub fn ip_address(&self) -> &str {
        &self.ip_address
    }

    // Metadata access
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn last_used_at(&self) -> DateTime<Utc> {
        self.last_used_at
    }

    // Behavior methods
    pub fn refresh(&mut self, new_duration: Duration) {
        self.access_token = format!("access_{}", Uuid::new_v4());
        self.expires_at = Utc::now() + new_duration;
        self.last_used_at = Utc::now();
    }

    pub fn revoke(&mut self) {
        self.revoked = true;
    }

    pub fn update_usage(&mut self) {
        self.last_used_at = Utc::now();
    }

    // Validation methods
    pub fn is_valid(&self) -> bool {
        !self.revoked && Utc::now() < self.expires_at
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn time_until_expiry(&self) -> Duration {
        self.expires_at - Utc::now()
    }
}

// Identity-based equality
impl PartialEq for AuthSession {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
    }
}

impl Eq for AuthSession {}
