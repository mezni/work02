#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::Utc;
    use configurator_service::domain::enums::network_type::NetworkType;
    use configurator_service::domain::models::network::Network;
    use configurator_service::domain::repositories::network_repository::{
        NetworkRepository, RepositoryResult,
    };
    use uuid::Uuid;

    // Mock repository for testing the trait
    struct MockNetworkRepository {
        networks: std::sync::RwLock<Vec<Network>>,
    }

    impl MockNetworkRepository {
        fn new() -> Self {
            Self {
                networks: std::sync::RwLock::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl NetworkRepository for MockNetworkRepository {
        async fn save(&self, network: &Network) -> RepositoryResult<()> {
            let mut networks = self.networks.write().unwrap();
            // Remove existing if present
            networks.retain(|n| n.network_id != network.network_id);
            networks.push(network.clone());
            Ok(())
        }

        async fn find_by_id(&self, network_id: Uuid) -> RepositoryResult<Option<Network>> {
            let networks = self.networks.read().unwrap();
            Ok(networks
                .iter()
                .find(|n| n.network_id == network_id)
                .cloned())
        }

        async fn delete(&self, network_id: Uuid) -> RepositoryResult<()> {
            let mut networks = self.networks.write().unwrap();
            networks.retain(|n| n.network_id != network_id);
            Ok(())
        }

        async fn list(&self) -> RepositoryResult<Vec<Network>> {
            let networks = self.networks.read().unwrap();
            Ok(networks.clone())
        }
    }

    #[tokio::test]
    async fn test_save_and_find_network() {
        let repo = MockNetworkRepository::new();
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let network = Network::new(
            network_id,
            Some("Test Network".to_string()),
            NetworkType::Individual,
            user_id,
            Utc::now(),
        );

        // Save network
        repo.save(&network).await.unwrap();

        // Find network
        let found = repo.find_by_id(network_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().network_id, network_id);
    }

    #[tokio::test]
    async fn test_save_updates_existing_network() {
        let repo = MockNetworkRepository::new();
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let mut network = Network::new(
            network_id,
            Some("Original Name".to_string()),
            NetworkType::Individual,
            user_id,
            Utc::now(),
        );

        // Save original
        repo.save(&network).await.unwrap();

        // Update network name and save again
        network.name = Some("Updated Name".to_string());
        repo.save(&network).await.unwrap();

        // Verify update
        let found = repo.find_by_id(network_id).await.unwrap().unwrap();
        assert_eq!(found.name, Some("Updated Name".to_string()));
    }

    #[tokio::test]
    async fn test_delete_network() {
        let repo = MockNetworkRepository::new();
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let network = Network::new(
            network_id,
            Some("Test Network".to_string()),
            NetworkType::Individual,
            user_id,
            Utc::now(),
        );

        // Save then delete
        repo.save(&network).await.unwrap();
        assert!(repo.find_by_id(network_id).await.unwrap().is_some());

        repo.delete(network_id).await.unwrap();
        assert!(repo.find_by_id(network_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_networks() {
        let repo = MockNetworkRepository::new();

        // Create multiple networks
        for i in 0..3 {
            let network = Network::new(
                Uuid::new_v4(),
                Some(format!("Network {}", i)),
                NetworkType::Individual,
                Uuid::new_v4(),
                Utc::now(),
            );
            repo.save(&network).await.unwrap();
        }

        let networks = repo.list().await.unwrap();
        assert_eq!(networks.len(), 3);
    }

    #[tokio::test]
    async fn test_exists_method() {
        let repo = MockNetworkRepository::new();
        let network_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let network = Network::new(
            network_id,
            Some("Test Network".to_string()),
            NetworkType::Individual,
            user_id,
            Utc::now(),
        );

        // Check non-existent
        assert!(!repo.exists(network_id).await.unwrap());

        // Save and check exists
        repo.save(&network).await.unwrap();
        assert!(repo.exists(network_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_find_non_existent_network() {
        let repo = MockNetworkRepository::new();
        let non_existent_id = Uuid::new_v4();

        let found = repo.find_by_id(non_existent_id).await.unwrap();
        assert!(found.is_none());
    }
}
