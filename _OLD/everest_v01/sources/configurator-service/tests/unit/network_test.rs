#[cfg(test)]
mod tests {
    use chrono::Utc;
    use configurator_service::domain::enums::network_type::NetworkType;
    use uuid::Uuid;
    //    use configurator_service::domain::events::network_events::{NetworkEvent, NetworkCreated};
    use configurator_service::domain::models::network::Network;

    #[test]
    fn test_network_creation() {
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let created_at = Utc::now();

        let network = Network::new(
            network_id,
            Some("Test Network".to_string()),
            NetworkType::Individual,
            user_id,
            created_at,
        );

        assert_eq!(network.network_id, network_id);
        assert_eq!(network.name.unwrap(), "Test Network");
        assert_eq!(network.network_type, NetworkType::Individual);
        assert!(!network.is_verified);
        assert!(network.is_active);
        assert!(network.is_live);
        assert_eq!(network.events.len(), 1);
    }

    #[test]
    fn test_network_verification() {
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let created_at = Utc::now();

        let mut network = Network::new(
            network_id,
            Some("Test Network".to_string()),
            NetworkType::Company,
            user_id,
            created_at,
        );

        let verifier_id = Uuid::new_v4();
        network.verify(verifier_id).unwrap();

        assert!(network.is_verified);
        assert_eq!(network.updated_by, Some(verifier_id));
        assert!(network.updated_at.is_some());
        assert_eq!(network.events.len(), 2); // Created + Verified
    }
}
