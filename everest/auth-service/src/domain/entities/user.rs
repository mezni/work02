use crate::domain::value_objects::{Email, OrganisationId, UserId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Option<UserId>,
    pub keycloak_id: String,
    pub organisation_id: Option<OrganisationId>,
    pub email: Email,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub status: UserStatus,
    pub is_live: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Pending,
    PendingPasswordReset,
}

impl User {
    pub fn new(
        keycloak_id: String,
        username: String,
        email: Email,
        first_name: String,
        last_name: String,
        role: String,
    ) -> Self {
        Self {
            id: None,
            keycloak_id,
            organisation_id: None,
            email,
            username,
            first_name,
            last_name,
            role,
            status: UserStatus::Pending,
            is_live: true,
            enabled: true,
        }
    }

    pub fn with_id(mut self, id: UserId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_organisation(mut self, organisation_id: OrganisationId) -> Self {
        self.organisation_id = Some(organisation_id);
        self
    }

    pub fn activate(&mut self) {
        self.status = UserStatus::Active;
        self.enabled = true;
    }

    pub fn deactivate(&mut self) {
        self.status = UserStatus::Inactive;
        self.enabled = false;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn set_password_reset_pending(&mut self) {
        self.status = UserStatus::PendingPasswordReset;
    }

    pub fn update_role(&mut self, role: String) {
        self.role = role;
    }

    pub fn assign_to_organisation(&mut self, organisation_id: OrganisationId) {
        self.organisation_id = Some(organisation_id);
    }

    pub fn remove_from_organisation(&mut self) {
        self.organisation_id = None;
    }
}