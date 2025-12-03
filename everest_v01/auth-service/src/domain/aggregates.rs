use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::user::User;
use super::token::Token;

#[derive(Debug)]
pub struct AuthSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user: User,
    pub token: Token,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

impl AuthSession {
    pub fn new(user: User, token: Token, ip_address: Option<String>, user_agent: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id: user.id,
            user,
            token,
            ip_address,
            user_agent,
            created_at: now,
            last_active_at: now,
            expires_at: token.expires_at,
            is_revoked: false,
        }
    }
    
    pub fn update_activity(&mut self) {
        self.last_active_at = Utc::now();
    }
    
    pub fn revoke(&mut self) {
        self.is_revoked = true;
    }
    
    pub fn is_valid(&self) -> bool {
        !self.is_revoked && !self.token.is_expired() && Utc::now() < self.expires_at
    }
    
    pub fn time_remaining(&self) -> chrono::Duration {
        let now = Utc::now();
        if now > self.expires_at {
            chrono::Duration::zero()
        } else {
            self.expires_at - now
        }
    }
}

#[derive(Debug)]
pub struct RegistrationAggregate {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub verification_token: Option<String>,
    pub verification_token_expires: Option<DateTime<Utc>>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub attempts: u32,
    pub max_attempts: u32,
}

impl RegistrationAggregate {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            verification_token: None,
            verification_token_expires: None,
            is_verified: false,
            created_at: Utc::now(),
            attempts: 0,
            max_attempts: 3,
        }
    }
    
    pub fn set_verification_token(&mut self, token: String, expires_in_hours: i64) {
        self.verification_token = Some(token);
        self.verification_token_expires = Some(Utc::now() + chrono::Duration::hours(expires_in_hours));
    }
    
    pub fn verify(&mut self, token: &str) -> bool {
        if let Some(verification_token) = &self.verification_token {
            if verification_token == token {
                if let Some(expires) = self.verification_token_expires {
                    if Utc::now() < expires {
                        self.is_verified = true;
                        self.verification_token = None;
                        self.verification_token_expires = None;
                        return true;
                    }
                }
            }
        }
        false
    }
    
    pub fn increment_attempts(&mut self) -> bool {
        self.attempts += 1;
        self.attempts >= self.max_attempts
    }
    
    pub fn reset_attempts(&mut self) {
        self.attempts = 0;
    }
    
    pub fn is_locked(&self) -> bool {
        self.attempts >= self.max_attempts
    }
}