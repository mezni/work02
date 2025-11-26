// src/domain/entities/user.rs
use crate::domain::events::{UserCreatedEvent, UserDeactivatedEvent, UserUpdatedEvent};
use crate::domain::value_objects::{Email, PhoneNumber, UserId, Username};
use crate::domain::DomainError;

#[derive(Debug, Clone)]
#[allow(dead_code)] // Remove this when all fields are used
pub struct User {
    id: UserId,
    keycloak_id: String,
    username: Username,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: PhoneNumber,
    avatar_url: Option<String>,
    is_active: bool,
    is_email_verified: bool,
    last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

// Rest of the implementation remains the same...
impl User {
    pub fn create(
        keycloak_id: String,
        username: Username,
        email: Email,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<(Self, UserCreatedEvent), DomainError> {
        let now = chrono::Utc::now();
        let user = User {
            id: UserId::generate(),
            keycloak_id,
            username,
            email,
            first_name,
            last_name,
            phone_number: PhoneNumber::new("")?,
            avatar_url: None,
            is_active: true,
            is_email_verified: false,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        };

        let event =
            UserCreatedEvent::new(user.id.clone(), user.email.clone(), user.username.clone());

        Ok((user, event))
    }
    // src/domain/entities/user.rs - update the update_profile method
    pub fn update_profile(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    ) -> Result<UserUpdatedEvent, DomainError> {
        let old_email = self.email.clone();

        // Validate phone number FIRST before updating any fields
        let new_phone_number = if let Some(phone) = &phone_number {
            Some(PhoneNumber::new(phone)?)
        } else {
            None
        };

        // Only update if Some value is provided
        if let Some(first_name) = first_name {
            self.first_name = Some(first_name);
        }

        if let Some(last_name) = last_name {
            self.last_name = Some(last_name);
        }

        if let Some(phone) = new_phone_number {
            self.phone_number = phone;
        }

        self.updated_at = chrono::Utc::now();

        Ok(UserUpdatedEvent::new(self.id.clone(), old_email))
    }

    pub fn mark_email_verified(&mut self) {
        self.is_email_verified = true;
        self.updated_at = chrono::Utc::now();
    }

    pub fn record_login(&mut self) {
        self.last_login_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }

    pub fn deactivate(&mut self) -> Result<UserDeactivatedEvent, DomainError> {
        if !self.is_active {
            return Err(DomainError::BusinessRule(
                "User already deactivated".to_string(),
            ));
        }

        self.is_active = false;
        self.updated_at = chrono::Utc::now();

        Ok(UserDeactivatedEvent::new(self.id.clone()))
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = chrono::Utc::now();
    }

    // Getters
    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn keycloak_id(&self) -> &str {
        &self.keycloak_id
    }
    pub fn username(&self) -> &Username {
        &self.username
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }
    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn is_email_verified(&self) -> bool {
        self.is_email_verified
    }
    pub fn last_login_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.last_login_at
    }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
