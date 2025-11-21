use crate::domain::models::person::Person;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    pub first_name: String,
    pub last_name: String,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub persons: Vec<Person>,
    pub is_live: bool,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Individual {
    pub fn new(
        first_name: String,
        last_name: String,
        support_email: Option<String>,
        support_phone: Option<String>,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            support_email,
            support_phone,
            persons: vec![],
            is_live: true,
            is_verified: false,
            is_active: true,
            created_by,
            updated_by: created_by,
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.persons.push(person);
    }

    pub fn verify(&mut self, updater: Option<Uuid>) {
        self.is_verified = true;
        self.updated_by = updater;
    }

    pub fn activate(&mut self, updater: Option<Uuid>) {
        self.is_active = true;
        self.updated_by = updater;
    }

    pub fn deactivate(&mut self, updater: Option<Uuid>) {
        self.is_active = false;
        self.updated_by = updater;
    }
}
