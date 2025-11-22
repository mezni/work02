use crate::domain::enums::network_type::NetworkType;
use crate::domain::events::network_events::{NetworkEvent, NetworkCreated, NetworkVerified};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    #[serde(with = "uuid::serde::compact")]
    pub network_id: Uuid,
    pub name: Option<String>,
    pub network_type: NetworkType,
    pub is_verified: bool,
    pub is_active: bool,
    pub is_live: bool,

    #[serde(with = "uuid::serde::compact")]
    pub created_by: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Uuid>,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", with = "chrono::serde::ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    
    #[serde(skip)]
    pub events: Vec<NetworkEvent>,
}

impl Network {
    pub fn new(
        network_id: Uuid,
        name: Option<String>,
        network_type: NetworkType,
        created_by: Uuid,
        created_at: DateTime<Utc>,
    ) -> Self {
        let mut network = Self {
            network_id,
            name: name.clone(),
            network_type: network_type.clone(),
            is_verified: false,
            is_active: true,
            is_live: true,
            created_by,
            updated_by: None,
            created_at,
            updated_at: None,
            events: vec![],
        };

        network.events.push(NetworkEvent::NetworkCreated(NetworkCreated {
            network_id,
            name,
            network_type,
            created_by,
            created_at,
        }));

        network
    }

    pub fn add_event(&mut self, event: NetworkEvent) {
        self.events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<NetworkEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn verify(&mut self, verified_by: Uuid) -> Result<(), String> {
        if self.is_verified {
            return Err("Network already verified".to_string());
        }

        self.is_verified = true;
        self.updated_by = Some(verified_by);
        self.updated_at = Some(Utc::now());

        self.events.push(NetworkEvent::NetworkVerified(NetworkVerified {
            network_id: self.network_id,
            verified_by,
            verified_at: Utc::now(),
        }));

        Ok(())
    }
}