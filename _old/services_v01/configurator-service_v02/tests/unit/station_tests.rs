use uuid::Uuid;
use crate::domain::models::{
    station::{Station, OperationalStatus},
    connector::Connector,
    connector_type::ConnectorType,
};
use crate::domain::events::station_events::StationEvent;

#[test]
fn test_station_creation() {
    let network_id = Uuid::new_v4();
    let created_by = Some(Uuid::new_v4());
    let station = Station::new(network_id, "Station 1".into(), Some("Downtown".into()), created_by);

    assert_eq!(station.name, "Station 1");
    assert_eq!(station.location, Some("Downtown".into()));
    assert!(station.is_live);
    assert_eq!(station.operational_status, OperationalStatus::Commissioning);
    assert_eq!(station.events.len(), 1);
    matches!(station.events[0], StationEvent::StationCreated { .. });
}

#[test]
fn test_station_activate_deactivate() {
    let network_id = Uuid::new_v4();
    let mut station = Station::new(network_id, "Station 2".into(), None, None);

    station.activate();
    assert_eq!(station.operational_status, OperationalStatus::Active);

    station.deactivate();
    assert_eq!(station.operational_status, OperationalStatus::OutOfService);
}

#[test]
fn test_add_connector() {
    let network_id = Uuid::new_v4();
    let mut station = Station::new(network_id, "Station 3".into(), None, None);

    let connector = Connector::new("C1".into(), ConnectorType::CCS);
    station.add_connector(connector.clone());

    assert_eq!(station.connectors.len(), 1);
    assert_eq!(station.connectors[0].id, connector.id);
    matches!(station.events.last().unwrap(), StationEvent::ConnectorAdded { .. });
}

#[test]
fn test_update_tags() {
    let network_id = Uuid::new_v4();
    let mut station = Station::new(network_id, "Station 4".into(), None, None);

    station.update_tags(vec!["fast".into(), "solar".into()]);
    assert_eq!(station.tags, vec!["fast", "solar"]);
}
