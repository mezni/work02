use crate::domain::enums::RoleType;
use crate::domain::value_objects::{email::Email, phone::Phone};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum PersonError {
    #[error("Person must have at least one contact method (email or phone)")]
    NoContactMethod,
    #[error("Full name cannot be empty")]
    EmptyName,
    #[error("Invalid person data: {0}")]
    InvalidData(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    id: Uuid,
    individual_id: Option<Uuid>,
    company_id: Option<Uuid>,
    full_name: String,
    email: Option<Email>,
    phone: Option<Phone>,
    job_title: Option<String>,
    department: Option<String>,
    role_type: RoleType,
    is_verified: bool,
    is_active: bool,
    is_live: bool,
    created_by: Uuid,
    updated_by: Option<Uuid>,
}

impl Person {
    pub fn new(
        full_name: String,
        email: Option<Email>,
        phone: Option<Phone>,
        job_title: Option<String>,
        department: Option<String>,
        role_type: RoleType,
        created_by: Uuid,
    ) -> Result<Self, PersonError> {
        if full_name.trim().is_empty() {
            return Err(PersonError::EmptyName);
        }

        if email.is_none() && phone.is_none() {
            return Err(PersonError::NoContactMethod);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            individual_id: None,
            company_id: None,
            full_name: full_name.trim().to_string(),
            email,
            phone,
            job_title: job_title.map(|t| t.trim().to_string()).filter(|t| !t.is_empty()),
            department: department.map(|d| d.trim().to_string()).filter(|d| !d.is_empty()),
            role_type,
            is_verified: false,
            is_active: true,
            is_live: true,
            created_by,
            updated_by: None,
        })
    }

    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn individual_id(&self) -> Option<Uuid> { self.individual_id }
    pub fn company_id(&self) -> Option<Uuid> { self.company_id }
    pub fn full_name(&self) -> &str { &self.full_name }
    pub fn email(&self) -> Option<&Email> { self.email.as_ref() }
    pub fn phone(&self) -> Option<&Phone> { self.phone.as_ref() }
    pub fn job_title(&self) -> Option<&str> { self.job_title.as_deref() }
    pub fn department(&self) -> Option<&str> { self.department.as_deref() }
    pub fn role_type(&self) -> RoleType { self.role_type }
    pub fn is_verified(&self) -> bool { self.is_verified }
    pub fn is_active(&self) -> bool { self.is_active }
    pub fn is_live(&self) -> bool { self.is_live }
    pub fn created_by(&self) -> Uuid { self.created_by }
    pub fn updated_by(&self) -> Option<Uuid> { self.updated_by }

    // Business methods
    pub fn has_contact_method(&self) -> bool {
        self.email.is_some() || self.phone.is_some()
    }

    pub fn is_individual_person(&self) -> bool {
        self.individual_id.is_some() && self.company_id.is_none()
    }

    pub fn is_company_person(&self) -> bool {
        self.company_id.is_some() && self.individual_id.is_none()
    }

    pub fn is_orphaned(&self) -> bool {
        self.individual_id.is_none() && self.company_id.is_none()
    }

    pub fn assign_to_individual(&mut self, individual_id: Uuid) -> Result<(), PersonError> {
        if self.company_id.is_some() {
            return Err(PersonError::InvalidData(
                "Person already assigned to a company".into()
            ));
        }
        self.individual_id = Some(individual_id);
        self.updated_by = Some(self.created_by);
        Ok(())
    }

    pub fn assign_to_company(&mut self, company_id: Uuid, updated_by: Uuid) -> Result<(), PersonError> {
        if self.individual_id.is_some() {
            return Err(PersonError::InvalidData(
                "Person already assigned to an individual".into()
            ));
        }
        self.company_id = Some(company_id);
        self.updated_by = Some(updated_by);
        Ok(())
    }

    pub fn unassign(&mut self, updated_by: Uuid) {
        self.individual_id = None;
        self.company_id = None;
        self.updated_by = Some(updated_by);
    }

    pub fn verify(&mut self, verified_by: Uuid) {
        self.is_verified = true;
        self.updated_by = Some(verified_by);
    }

    pub fn unverify(&mut self, updated_by: Uuid) {
        self.is_verified = false;
        self.updated_by = Some(updated_by);
    }

    pub fn activate(&mut self, updated_by: Uuid) {
        self.is_active = true;
        self.updated_by = Some(updated_by);
    }

    pub fn deactivate(&mut self, updated_by: Uuid) {
        self.is_active = false;
        self.updated_by = Some(updated_by);
    }

    pub fn archive(&mut self, updated_by: Uuid) {
        self.is_live = false;
        self.updated_by = Some(updated_by);
    }

    pub fn restore(&mut self, updated_by: Uuid) {
        self.is_live = true;
        self.updated_by = Some(updated_by);
    }

    pub fn update_contact_info(
        &mut self,
        email: Option<Email>,
        phone: Option<Phone>,
        updated_by: Uuid,
    ) -> Result<(), PersonError> {
        if email.is_none() && phone.is_none() {
            return Err(PersonError::NoContactMethod);
        }
        self.email = email;
        self.phone = phone;
        self.updated_by = Some(updated_by);
        Ok(())
    }

    pub fn update_role(
        &mut self,
        role_type: RoleType,
        updated_by: Uuid,
    ) {
        self.role_type = role_type;
        self.updated_by = Some(updated_by);
    }

    pub fn update_job_info(
        &mut self,
        job_title: Option<String>,
        department: Option<String>,
        updated_by: Uuid,
    ) {
        self.job_title = job_title.map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
        self.department = department.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());
        self.updated_by = Some(updated_by);
    }

    pub fn can_manage_billing(&self) -> bool {
        matches!(self.role_type, RoleType::Admin | RoleType::Billing)
    }

    pub fn can_manage_operations(&self) -> bool {
        matches!(self.role_type, RoleType::Admin | RoleType::Operations | RoleType::Technical)
    }

    pub fn can_manage_technical(&self) -> bool {
        matches!(self.role_type, RoleType::Admin | RoleType::Technical)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}