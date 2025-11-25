use crate::domain::enums::UserRole;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Email;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: String,
    pub username: String,
    pub email: Email,
    pub role: UserRole,
    pub company_id: Option<Uuid>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        keycloak_id: String,
        username: String,
        email: Email,
        role: UserRole,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            keycloak_id,
            username,
            email,
            role,
            company_id: None,
            email_verified: false,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    pub fn is_partner(&self) -> bool {
        matches!(self.role, UserRole::Partner)
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.role, UserRole::Operator)
    }

    pub fn is_user(&self) -> bool {
        matches!(self.role, UserRole::User)
    }

    pub fn is_guest(&self) -> bool {
        matches!(self.role, UserRole::Guest)
    }

    pub fn has_company_access(&self, company_id: &Uuid) -> bool {
        if self.is_admin() {
            return true;
        }

        match self.company_id {
            Some(user_company_id) => &user_company_id == company_id,
            None => false,
        }
    }

    pub fn can_manage_user(&self, target_user: &User) -> bool {
        if self.is_admin() {
            return true;
        }

        if self.is_user() || self.is_guest() {
            return self.id == target_user.id;
        }

        // Partner/Operator can only manage users in their company
        if let (Some(self_company), Some(target_company)) =
            (self.company_id, target_user.company_id)
        {
            self_company == target_company
        } else {
            false
        }
    }

    pub fn assign_to_company(&mut self, company_id: Uuid) -> Result<(), DomainError> {
        if self.is_user() || self.is_guest() {
            return Err(DomainError::InvalidOperation(
                "User role cannot be assigned to a company".to_string(),
            ));
        }

        self.company_id = Some(company_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_from_company(&mut self) {
        self.company_id = None;
        self.updated_at = Utc::now();
    }

    pub fn change_role(&mut self, new_role: UserRole) -> Result<(), DomainError> {
        // Business rule: If changing from company role to user/guest, remove company assignment
        if (self.role.is_company_scoped() && !new_role.is_company_scoped())
            && self.company_id.is_some()
        {
            self.company_id = None;
        }

        // Business rule: If changing to company role without company, it's invalid
        if new_role.is_company_scoped() && self.company_id.is_none() {
            return Err(DomainError::InvalidOperation(
                "Company-scoped role requires company assignment".to_string(),
            ));
        }

        self.role = new_role;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_email_verified(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
    }
}
