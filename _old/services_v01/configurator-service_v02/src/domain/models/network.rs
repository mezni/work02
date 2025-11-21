use crate::domain::enums::network_type::NetworkType;
use crate::domain::models::{company::Company, individual::Individual};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub network_type: NetworkType,
    pub owner_individual: Option<Individual>,
    pub owner_company: Option<Company>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub is_live: bool,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Network {
    pub fn new_individual(
        individual: Individual,
        support_email: Option<String>,
        support_phone: Option<String>,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            network_type: NetworkType::INDIVIDUAL,
            owner_individual: Some(individual),
            owner_company: None,
            support_email,
            support_phone,
            is_live: true,
            is_verified: false,
            is_active: true,
            created_by,
            updated_by: created_by,
        }
    }

    pub fn new_company(
        company: Company,
        support_email: Option<String>,
        support_phone: Option<String>,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            network_type: NetworkType::COMPANY,
            owner_individual: None,
            owner_company: Some(company),
            support_email,
            support_phone,
            is_live: true,
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
}
