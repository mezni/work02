use crate::domain::models::connector_type::ConnectorType;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Connector {
    pub id: Uuid,
    pub name: String,
    pub connector_type: ConnectorType,
    pub status: ConnectorStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectorStatus {
    Available,
    Charging,
    Faulty,
    Reserved,
    Offline,
}

impl Connector {
    pub fn new(name: String, connector_type: ConnectorType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            connector_type,
            status: ConnectorStatus::Available,
        }
    }

    pub fn set_status(&mut self, status: ConnectorStatus) {
        self.status = status;
    }
}
