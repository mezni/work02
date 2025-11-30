use crate::domain::value_objects::OrganisationId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Organisation {
    pub id: Option<OrganisationId>,
    pub name: String,
    pub description: String,
    pub is_live: bool,
}

impl Organisation {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name,
            description,
            is_live: true,
        }
    }

    pub fn with_id(mut self, id: OrganisationId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn soft_delete(&mut self) {
        self.is_live = false;
    }

    pub fn restore(&mut self) {
        self.is_live = true;
    }
}