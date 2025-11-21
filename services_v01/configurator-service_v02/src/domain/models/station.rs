use chrono::Utc;
use uuid::Uuid;

use crate::domain::events::station_events::StationEvent;
use crate::domain::models::{connector::Connector, connector_type::ConnectorType};

#[derive(Debug, Clone)]
pub struct Station {
    pub id: Uuid,
    pub network_id: Uuid,
    pub name: String,
    pub location: Option<String>,
    pub tags: Vec<String>,
    pub operational_status: OperationalStatus,
    pub verification_status: VerificationStatus,
    pub is_live: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub connectors: Vec<Connector>,
    pub events: Vec<StationEvent>, // store domain events
}

#[derive(Debug, Clone)]
pub enum OperationalStatus {
    Active,
    Maintenance,
    OutOfService,
    Commissioning,
}

#[derive(Debug, Clone)]
pub enum VerificationStatus {
    Verified,
    Unverified,
}

impl Station {
    pub fn new(
        network_id: Uuid,
        name: String,
        location: Option<String>,
        created_by: Option<Uuid>,
    ) -> Self {
        let id = Uuid::new_v4();
        let mut station = Self {
            id,
            network_id,
            name,
            location,
            tags: Vec::new(),
            operational_status: OperationalStatus::Commissioning,
            verification_status: VerificationStatus::Unverified,
            is_live: true,
            created_by,
            updated_by: created_by,
            connectors: Vec::new(),
            events: Vec::new(),
        };

        // Record creation event
        station.record_event(StationEvent::StationCreated {
            station_id: id,
            network_id,
            name: station.name.clone(),
        });

        station
    }

    pub fn activate(&mut self, updater: Option<Uuid>) {
        self.operational_status = OperationalStatus::Active;
        self.updated_by = updater;
    }

    pub fn deactivate(&mut self, updater: Option<Uuid>) {
        self.operational_status = OperationalStatus::OutOfService;
        self.updated_by = updater;
    }

    pub fn update_tags(&mut self, new_tags: Vec<String>, updater: Option<Uuid>) {
        self.tags = new_tags;
        self.updated_by = updater;
    }

    pub fn add_connector(&mut self, connector_type: ConnectorType, updater: Option<Uuid>) {
        let connector = Connector::new(connector_type);
        self.connectors.push(connector.clone());
        self.updated_by = updater;

        self.record_event(StationEvent::ConnectorAdded {
            station_id: self.id,
            connector_id: connector.id,
            connector_type: connector.connector_type,
        });
    }

    fn record_event(&mut self, event: StationEvent) {
        self.events.push(event);
    }
}
