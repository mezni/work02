#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;
    use configurator_service::domain::enums::network_type::NetworkType;
    use configurator_service::domain::events::network_events::{NetworkEvent, NetworkCreated, NetworkVerified};

    #[test]
    fn test_network_created_event() {
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let created_at = Utc::now();

        let event = NetworkCreated {
            network_id,
            name: Some("Test Network".to_string()),
            network_type: NetworkType::Company,
            created_by: user_id,
            created_at,
        };

        assert_eq!(event.network_id, network_id);
        assert_eq!(event.name.unwrap(), "Test Network");
        assert_eq!(event.network_type, NetworkType::Company);
    }

    #[test]
    fn test_network_verified_event() {
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let verified_at = Utc::now();

        let event = NetworkVerified {
            network_id,
            verified_by: user_id,
            verified_at,
        };

        assert_eq!(event.network_id, network_id);
        assert_eq!(event.verified_by, user_id);
    }

    #[test]
    fn test_network_event_enum() {
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let created_at = Utc::now();

        let created_event = NetworkEvent::NetworkCreated(NetworkCreated {
            network_id,
            name: Some("Test Network".to_string()),
            network_type: NetworkType::Individual,
            created_by: user_id,
            created_at,
        });

        match created_event {
            NetworkEvent::NetworkCreated(event) => {
                assert_eq!(event.network_id, network_id);
            }
            _ => panic!("Expected NetworkCreated event"),
        }
    }
}