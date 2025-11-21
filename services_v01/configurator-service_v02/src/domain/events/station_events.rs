use uuid::Uuid;
use crate::domain::models::connector_type::ConnectorType;

/// All domain events related to Station
#[derive(Debug, Clone)]
pub enum StationEvent {
    StationCreated {
        station_id: Uuid,
        network_id: Uuid,
        name: String,
    },
    StationActivated {
        station_id: Uuid,
    },
    StationDeactivated {
        station_id: Uuid,
    },
    ConnectorAdded {
        station_id: Uuid,
        connector_id: Uuid,
        connector_type: ConnectorType,
    },
    ConnectorUpdated {
        station_id: Uuid,
        connector_id: Uuid,
        connector_type: ConnectorType,
    },
    ConnectorDeactivated {
        station_id: Uuid,
        connector_id: Uuid,
    },
    ConnectorReactivated {
        station_id: Uuid,
        connector_id: Uuid,
    },
    TagsUpdated {
        station_id: Uuid,
        tags: Vec<String>,
    },
}

impl StationEvent {
    /// Optional helper: return station ID for the event
    pub fn station_id(&self) -> Uuid {
        match self {
            StationEvent::StationCreated { station_id, .. } => *station_id,
            StationEvent::StationActivated { station_id } => *station_id,
            StationEvent::StationDeactivated { station_id } => *station_id,
            StationEvent::ConnectorAdded { station_id, .. } => *station_id,
            StationEvent::ConnectorUpdated { station_id, .. } => *station_id,
            StationEvent::ConnectorDeactivated { station_id, .. } => *station_id,
            StationEvent::ConnectorReactivated { station_id, .. } => *station_id,
            StationEvent::TagsUpdated { station_id, .. } => *station_id,
        }
    }
}
