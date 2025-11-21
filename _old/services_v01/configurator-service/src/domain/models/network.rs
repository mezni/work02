use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::domain::enums::network_type::NetworkType;
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::phone::Phone;

use crate::domain::events::{
    network_created::NetworkCreated,
    network_activated::NetworkActivated,
    network_deactivated::NetworkDeactivated,
    network_verified::NetworkVerified,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub network_id: Uuid,
    pub name: Option<String>,
    pub network_type: NetworkType,

    pub support_email: Option<Email>,
    pub support_phone: Option<Phone>,

    pub is_verified: bool,
    pub is_active: bool,
    pub is_live: bool,

    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Network {
    pub fn create(
        name: Option<String>,
        network_type: NetworkType,
        email: Option<Email>,
        phone: Option<Phone>,
        created_by: Uuid,
    ) -> (Self, NetworkCreated) {

        let network = Self {
            network_id: Uuid::new_v4(),
            name,
            network_type,
            support_email: email,
            support_phone: phone,
            is_verified: false,
            is_active: true,
            is_live: true,
            created_by,
            updated_by: None,
            created_at: Utc::now(),
            updated_at: None,
        };

        let event = NetworkCreated {
            network_id: network.network_id,
            created_by,
            created_at: network.created_at,
        };

        (network, event)
    }

    pub fn activate(&mut self, user_id: Uuid) -> Result<NetworkActivated, String> {
        if self.is_active {
            return Err("Network already active".into());
        }

        self.is_active = true;
        self.updated_by = Some(user_id);
        self.updated_at = Some(Utc::now());

        Ok(NetworkActivated {
            network_id: self.network_id,
            activated_by: user_id,
            activated_at: self.updated_at.unwrap(),
        })
    }

    pub fn deactivate(&mut self, user_id: Uuid) -> Result<NetworkDeactivated, String> {
        if !self.is_active {
            return Err("Network already inactive".into());
        }

        self.is_active = false;
        self.updated_by = Some(user_id);
        self.updated_at = Some(Utc::now());

        Ok(NetworkDeactivated {
            network_id: self.network_id,
            deactivated_by: user_id,
            deactivated_at: self.updated_at.unwrap(),
        })
    }

    pub fn verify(&mut self, user_id: Uuid) -> Result<NetworkVerified, String> {
        if self.is_verified {
            return Err("Network already verified".into());
        }

        self.is_verified = true;
        self.updated_by = Some(user_id);
        self.updated_at = Some(Utc::now());

        Ok(NetworkVerified {
            network_id: self.network_id,
            verified_by: user_id,
            verified_at: self.updated_at.unwrap(),
        })
    }

    pub fn update_contact_info(
        &mut self,
        email: Option<Email>,
        phone: Option<Phone>,
        updated_by: Uuid
    ) {
        self.support_email = email;
        self.support_phone = phone;
        self.updated_by = Some(updated_by);
        self.updated_at = Some(Utc::now());
    }
}
