use crate::domain::enums::company_type::CompanyType;
use crate::domain::models::person::Person;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub company_type: CompanyType,
    pub company_size: Option<String>,
    pub website: Option<String>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub persons: Vec<Person>,
    pub is_live: bool,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Company {
    pub fn new(
        name: String,
        company_type: CompanyType,
        company_size: Option<String>,
        website: Option<String>,
        support_email: Option<String>,
        support_phone: Option<String>,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            name,
            company_type,
            company_size,
            website,
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
