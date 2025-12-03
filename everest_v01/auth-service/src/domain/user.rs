use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::value_objects::{UserRole, CompanyName, StationName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub company_name: String,
    pub station_name: String,
    pub is_active: bool,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            role: UserRole::User,
            company_name: String::new(),
            station_name: String::new(),
            is_active: true,
            email_verified: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn builder(email: String) -> UserBuilder {
        UserBuilder::new(email)
    }
    
    pub fn update_profile(&mut self, company_name: Option<String>, station_name: Option<String>) {
        if let Some(cn) = company_name {
            self.company_name = cn;
        }
        if let Some(sn) = station_name {
            self.station_name = sn;
        }
        self.updated_at = Utc::now();
    }
    
    pub fn update_role(&mut self, role: UserRole) {
        self.role = role;
        self.updated_at = Utc::now();
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }
    
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }
    
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
    }
    
    pub fn has_permission(&self, required_role: UserRole) -> bool {
        match required_role {
            UserRole::User => true, // All users have user permissions
            UserRole::Manager => matches!(self.role, UserRole::Manager | UserRole::Admin),
            UserRole::Admin => matches!(self.role, UserRole::Admin),
        }
    }
}

pub struct UserBuilder {
    user: User,
}

impl UserBuilder {
    pub fn new(email: String) -> Self {
        Self {
            user: User::new(email),
        }
    }
    
    pub fn with_role(mut self, role: UserRole) -> Self {
        self.user.role = role;
        self
    }
    
    pub fn with_company_name(mut self, company_name: String) -> Self {
        self.user.company_name = company_name;
        self
    }
    
    pub fn with_station_name(mut self, station_name: String) -> Self {
        self.user.station_name = station_name;
        self
    }
    
    pub fn with_email_verified(mut self, verified: bool) -> Self {
        self.user.email_verified = verified;
        self
    }
    
    pub fn build(self) -> User {
        self.user
    }
}