use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::value_objects::UserRole;

pub trait DomainEvent: Send + Sync {
    fn event_id(&self) -> Uuid;
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn aggregate_id(&self) -> Uuid;
    fn version(&self) -> u32;
    fn metadata(&self) -> &EventMetadata;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub correlation_id: Uuid,
    pub causation_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub source: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub company_name: String,
    pub station_name: String,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
    pub version: u32,
}

impl UserRegistered {
    pub fn new(
        user_id: Uuid,
        email: String,
        role: UserRole,
        company_name: String,
        station_name: String,
        metadata: EventMetadata,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            user_id,
            email,
            role,
            company_name,
            station_name,
            occurred_at: Utc::now(),
            metadata,
            version: 1,
        }
    }
}

impl DomainEvent for UserRegistered {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    
    fn event_type(&self) -> &str {
        "UserRegistered"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
    
    fn version(&self) -> u32 {
        self.version
    }
    
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedIn {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
    pub version: u32,
}

impl UserLoggedIn {
    pub fn new(
        user_id: Uuid,
        email: String,
        role: UserRole,
        ip_address: Option<String>,
        user_agent: Option<String>,
        metadata: EventMetadata,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            user_id,
            email,
            role,
            ip_address,
            user_agent,
            occurred_at: Utc::now(),
            metadata,
            version: 1,
        }
    }
}

impl DomainEvent for UserLoggedIn {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    
    fn event_type(&self) -> &str {
        "UserLoggedIn"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
    
    fn version(&self) -> u32 {
        self.version
    }
    
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenIssued {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub token_id: Uuid,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
    pub version: u32,
}

impl TokenIssued {
    pub fn new(
        user_id: Uuid,
        token_id: Uuid,
        token_type: String,
        expires_at: DateTime<Utc>,
        metadata: EventMetadata,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            user_id,
            token_id,
            token_type,
            expires_at,
            occurred_at: Utc::now(),
            metadata,
            version: 1,
        }
    }
}

impl DomainEvent for TokenIssued {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    
    fn event_type(&self) -> &str {
        "TokenIssued"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
    
    fn version(&self) -> u32 {
        self.version
    }
    
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
}