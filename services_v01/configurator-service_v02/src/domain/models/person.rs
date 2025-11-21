use crate::domain::enums::role_type::RoleType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub role_type: RoleType,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Person {
    pub fn new(
        first_name: String,
        last_name: String,
        role_type: RoleType,
        email: Option<String>,
        phone: Option<String>,
        job_title: Option<String>,
        department: Option<String>,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            role_type,
            email,
            phone,
            job_title,
            department,
            is_verified: false,
            is_active: true,
            created_by,
            updated_by: created_by,
        }
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

    pub fn update_contact(
        &mut self,
        email: Option<String>,
        phone: Option<String>,
        updater: Option<Uuid>,
    ) {
        self.email = email;
        self.phone = phone;
        self.updated_by = updater;
    }

    pub fn update_position(
        &mut self,
        job_title: Option<String>,
        department: Option<String>,
        updater: Option<Uuid>,
    ) {
        self.job_title = job_title;
        self.department = department;
        self.updated_by = updater;
    }
}
