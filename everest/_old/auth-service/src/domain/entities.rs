use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use super::value_objects::{Email, OrganisationName, Role};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub keycloak_id: String,
    pub email: Email,
    pub username: String,
    pub role: Role,
    pub organisation_name: Option<OrganisationName>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        keycloak_id: String,
        email: Email,
        username: String,
        role: Role,
        organisation_name: Option<OrganisationName>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            keycloak_id,
            email,
            username,
            role,
            organisation_name,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn validate_operator_has_organisation(&self) -> Result<(), String> {
        if self.role == Role::Operator && self.organisation_name.is_none() {
            return Err("Operator must belong to an organisation".to_string());
        }
        Ok(())
    }

    pub fn can_access_organisation(&self, org_name: &str) -> bool {
        match &self.role {
            Role::Admin => true,
            Role::Partner | Role::Operator => {
                self.organisation_name
                    .as_ref()
                    .map(|on| on.value() == org_name)
                    .unwrap_or(false)
            }
        }
    }
}
