use crate::domain::value_objects::{Email, Password, UserId};
use chrono::{DateTime, Utc};
use std::collections::HashSet;

/// User Entity - has identity (UserId) and lifecycle
#[derive(Debug, Clone)]
pub struct User {
    // Identity
    id: UserId,

    // Core attributes
    email: Email,
    password_hash: String,
    first_name: String,
    last_name: String,
    username: Option<String>,

    // Status and metadata
    email_verified: bool,
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,

    // Roles
    roles: HashSet<String>,
}

impl User {
    /// Create a new user entity
    pub fn new(
        email: Email,
        password: Password,
        first_name: String,
        last_name: String,
        username: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            password_hash: password.hash(),
            first_name,
            last_name,
            username,
            email_verified: false,
            active: true,
            created_at: now,
            updated_at: now,
            last_login_at: None,
            roles: HashSet::new(),
        }
    }

    // Identity access
    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    // Core attributes access
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    // Status access
    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn last_login_at(&self) -> Option<DateTime<Utc>> {
        self.last_login_at
    }

    // Roles access
    pub fn roles(&self) -> &HashSet<String> {
        &self.roles
    }

    // Behavior methods
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
    }

    pub fn update_profile(
        &mut self,
        first_name: String,
        last_name: String,
        username: Option<String>,
    ) {
        self.first_name = first_name;
        self.last_name = last_name;
        self.username = username;
        self.updated_at = Utc::now();
    }

    pub fn change_password(&mut self, new_password: Password) {
        self.password_hash = new_password.hash();
        self.updated_at = Utc::now();
    }

    pub fn record_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.updated_at = Utc::now();
    }

    // Role management
    pub fn assign_role(&mut self, role: String) {
        self.roles.insert(role);
        self.updated_at = Utc::now();
    }

    pub fn revoke_role(&mut self, role: &str) -> bool {
        let removed = self.roles.remove(role);
        if removed {
            self.updated_at = Utc::now();
        }
        removed
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }

    // Validation methods
    pub fn can_login(&self) -> bool {
        self.active && self.email_verified
    }

    pub fn verify_password(&self, password: &Password) -> bool {
        password.verify(&self.password_hash)
    }

    // Identity-based equality
    pub fn same_identity(&self, other: &User) -> bool {
        self.id == other.id
    }
}

// Identity-based equality
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}
