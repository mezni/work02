use crate::domain::value_objects::{Email, Username, Role};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: Username,
    pub email: Email,
    pub password_hash: String,
    pub role: Role,
    pub organisation_id: Option<Uuid>,
    pub station_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl User {
    pub fn new(
        username: Username,
        email: Email,
        password_hash: String,
        role: Role,
    ) -> Self {
        let now = Utc::now();
        User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            role,
            organisation_id: None,
            station_id: None,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
    
    pub fn register(
        username: Username,
        email: Email,
        password_hash: String,
    ) -> Self {
        Self::new(username, email, password_hash, Role::RegisteredUser)
    }
    
    pub fn promote_to_partner(&mut self, organisation_id: Uuid) {
        self.role = Role::Partner;
        self.organisation_id = Some(organisation_id);
        self.station_id = None;
        self.updated_at = Utc::now();
    }

    pub fn promote_to_operator(&mut self, organisation_id: Uuid, station_id: Uuid) {
        self.role = Role::Operator;
        self.organisation_id = Some(organisation_id);
        self.station_id = Some(station_id);
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, &self.password_hash)
    }
}
