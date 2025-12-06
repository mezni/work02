use async_trait::async_trait;
use crate::domain::entities::*;
use crate::infrastructure::error::DomainError;

#[async_trait]
pub trait NetworkRepository: Send + Sync {
    async fn create(&self, network_id: &str, name: &str, network_type: &str, 
                   support_phone: Option<&str>, support_email: Option<&str>, 
                   created_by: &str) -> Result<Network, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Network>, DomainError>;
    async fn list_all(&self) -> Result<Vec<Network>, DomainError>;
    async fn list_by_network_id(&self, network_id: &str) -> Result<Vec<Network>, DomainError>;
    async fn update(&self, id: &str, name: Option<&str>, support_phone: Option<&str>, 
                   support_email: Option<&str>, updated_by: &str) -> Result<Network, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait StationRepository: Send + Sync {
    async fn create(&self, station_id: &str, network_id: &str, name: &str,
                   address: Option<&str>, latitude: Option<f64>, longitude: Option<f64>,
                   operational_status: &str, created_by: &str) -> Result<Station, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Station>, DomainError>;
    async fn list_all(&self) -> Result<Vec<Station>, DomainError>;
    async fn list_by_network_id(&self, network_id: &str) -> Result<Vec<Station>, DomainError>;
    async fn list_by_station_id(&self, station_id: &str) -> Result<Vec<Station>, DomainError>;
    async fn update(&self, id: &str, name: Option<&str>, address: Option<&str>, 
                   operational_status: Option<&str>, updated_by: &str) -> Result<Station, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ChargerRepository: Send + Sync {
    async fn create(&self, charger_id: &str, station_id: &str, serial_number: Option<&str>,
                   max_power_kw: Option<f64>, status: &str, created_by: &str) -> Result<Charger, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Charger>, DomainError>;
    async fn list_by_station_id(&self, station_id: &str) -> Result<Vec<Charger>, DomainError>;
    async fn list_by_network_id(&self, network_id: &str) -> Result<Vec<Charger>, DomainError>;
    async fn update(&self, id: &str, max_power_kw: Option<f64>, status: Option<&str>, 
                   updated_by: &str) -> Result<Charger, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ConnectorRepository: Send + Sync {
    async fn create(&self, connector_id: &str, charger_id: &str, station_id: &str,
                   connector_type_id: i32, connector_index: i32, capacity_kw: Option<f64>,
                   operational_status: &str, created_by: &str) -> Result<Connector, DomainError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Connector>, DomainError>;
    async fn list_by_charger_id(&self, charger_id: &str) -> Result<Vec<Connector>, DomainError>;
    async fn list_by_station_id(&self, station_id: &str) -> Result<Vec<Connector>, DomainError>;
    async fn update(&self, id: &str, capacity_kw: Option<f64>, operational_status: Option<&str>,
                   updated_by: &str) -> Result<Connector, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
}