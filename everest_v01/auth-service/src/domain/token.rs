use serde::{Serialize, Deserialize};
use jsonwebtoken::{Header, Algorithm};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use super::value_objects::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Token {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::seconds(expires_in);
        
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
            issued_at,
            expires_at,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    pub fn time_until_expiry(&self) -> Duration {
        let now = Utc::now();
        if now > self.expires_at {
            Duration::zero()
        } else {
            self.expires_at - now
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    // Standard JWT claims
    pub sub: String,        // Subject (user ID)
    pub iss: String,        // Issuer
    pub aud: String,        // Audience
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub jti: Uuid,          // JWT ID
    
    // Custom claims
    pub email: String,
    pub role: String,
    pub company_name: String,
    pub station_name: String,
    pub is_active: bool,
    pub email_verified: bool,
}

impl TokenClaims {
    pub fn new(
        user_id: Uuid,
        email: String,
        role: UserRole,
        company_name: String,
        station_name: String,
        is_active: bool,
        email_verified: bool,
        issuer: String,
        audience: String,
        expires_in_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expires_in_hours);
        
        Self {
            sub: user_id.to_string(),
            iss: issuer,
            aud: audience,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4(),
            email,
            role: role.as_str().to_string(),
            company_name,
            station_name,
            is_active,
            email_verified,
        }
    }
    
    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }
    
    pub fn role(&self) -> super::value_objects::UserRole {
        super::value_objects::UserRole::from_str(&self.role)
            .unwrap_or(super::value_objects::UserRole::User)
    }
    
    pub fn is_valid(&self) -> bool {
        let now = Utc::now().timestamp();
        now < self.exp && self.is_active && self.email_verified
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, token: String, expires_in_days: i64) -> Self {
        let created_at = Utc::now();
        let expires_at = created_at + Duration::days(expires_in_days);
        
        Self {
            token,
            user_id,
            expires_at,
            created_at,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}