use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use configurator_service::application::{
    CreateNetworkCommand, GetNetworkQuery, ListNetworksQuery, NetworkApplicationService,
    VerifyNetworkCommand,
};
use configurator_service::domain::enums::network_type::NetworkType;
use configurator_service::domain::models::network::Network;
use configurator_service::domain::repositories::network_repository::{
    NetworkRepository, RepositoryResult,
};
// Import ApplicationError from the correct path
use configurator_service::application::services::network_application_service::ApplicationError;

#[derive(Default)]
struct MockNetworkRepository {
    networks: Arc<RwLock<HashMap<Uuid, Network>>>,
}

impl MockNetworkRepository {
    fn new() -> Self {
        Self {
            networks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn with_initial_data(networks: Vec<Network>) -> Self {
        let mut map = HashMap::new();
        for network in networks {
            map.insert(network.network_id, network);
        }
        Self {
            networks: Arc::new(RwLock::new(map)),
        }
    }
}

#[async_trait]
impl NetworkRepository for MockNetworkRepository {
    async fn save(&self, network: &Network) -> RepositoryResult<()> {
        let mut networks = self.networks.write().await;
        networks.insert(network.network_id, network.clone());
        Ok(())
    }

    async fn find_by_id(&self, network_id: Uuid) -> RepositoryResult<Option<Network>> {
        let networks = self.networks.read().await;
        Ok(networks.get(&network_id).cloned())
    }

    async fn delete(&self, network_id: Uuid) -> RepositoryResult<()> {
        let mut networks = self.networks.write().await;
        networks.remove(&network_id);
        Ok(())
    }

    async fn list(&self) -> RepositoryResult<Vec<Network>> {
        let networks = self.networks.read().await;
        Ok(networks.values().cloned().collect())
    }
}

#[tokio::test]
async fn test_create_network_success() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);
    let user_id = Uuid::new_v4();

    let command = CreateNetworkCommand {
        name: Some("Test Network".to_string()),
        network_type: NetworkType::Company,
        created_by: user_id,
    };

    // Act
    let result = service.create_network(command).await;

    // Assert
    assert!(result.is_ok());
    let network_dto = result.unwrap();
    assert_eq!(network_dto.name, Some("Test Network".to_string()));
    assert_eq!(network_dto.network_type, NetworkType::Company);
    assert_eq!(network_dto.created_by, user_id);
    assert!(!network_dto.is_verified);
    assert!(network_dto.is_active);
    assert!(network_dto.is_live);
}

#[tokio::test]
async fn test_create_network_without_name() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);
    let user_id = Uuid::new_v4();

    let command = CreateNetworkCommand {
        name: None,
        network_type: NetworkType::Individual,
        created_by: user_id,
    };

    // Act
    let result = service.create_network(command).await;

    // Assert
    assert!(result.is_ok());
    let network_dto = result.unwrap();
    assert!(network_dto.name.is_none());
    assert_eq!(network_dto.network_type, NetworkType::Individual);
}

#[tokio::test]
async fn test_verify_network_success() {
    // Arrange
    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();

    let network = Network::new(
        network_id,
        Some("Test Network".to_string()),
        NetworkType::Company,
        user_id,
        Utc::now(),
    );

    let repository = MockNetworkRepository::with_initial_data(vec![network]);
    let service = NetworkApplicationService::new(repository);

    let command = VerifyNetworkCommand {
        network_id,
        verified_by: admin_id,
    };

    // Act
    let result = service.verify_network(command).await;

    // Assert
    assert!(result.is_ok());
    let network_dto = result.unwrap();
    assert!(network_dto.is_verified);
    assert_eq!(network_dto.updated_by, Some(admin_id));
    assert!(network_dto.updated_at.is_some());
}

#[tokio::test]
async fn test_verify_network_not_found() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);
    let admin_id = Uuid::new_v4();

    let command = VerifyNetworkCommand {
        network_id: Uuid::new_v4(), // Non-existent ID
        verified_by: admin_id,
    };

    // Act
    let result = service.verify_network(command).await;

    // Assert
    assert!(result.is_err());
    match result.unwrap_err() {
        ApplicationError::NotFound => (), // Expected
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_get_network_success() {
    // Arrange
    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let network = Network::new(
        network_id,
        Some("Test Network".to_string()),
        NetworkType::Individual,
        user_id,
        Utc::now(),
    );

    let repository = MockNetworkRepository::with_initial_data(vec![network]);
    let service = NetworkApplicationService::new(repository);

    let query = GetNetworkQuery { network_id };

    // Act
    let result = service.get_network(query).await;

    // Assert
    assert!(result.is_ok());
    let network_dto = result.unwrap();
    assert!(network_dto.is_some());
    let network_dto = network_dto.unwrap();
    assert_eq!(network_dto.network_id, network_id);
    assert_eq!(network_dto.name, Some("Test Network".to_string()));
}

#[tokio::test]
async fn test_get_network_not_found() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);

    let query = GetNetworkQuery {
        network_id: Uuid::new_v4(), // Non-existent ID
    };

    // Act
    let result = service.get_network(query).await;

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_list_networks() {
    // Arrange
    let user_id = Uuid::new_v4();
    let networks = vec![
        Network::new(
            Uuid::new_v4(),
            Some("Network 1".to_string()),
            NetworkType::Individual,
            user_id,
            Utc::now(),
        ),
        Network::new(
            Uuid::new_v4(),
            Some("Network 2".to_string()),
            NetworkType::Company,
            user_id,
            Utc::now(),
        ),
    ];

    let repository = MockNetworkRepository::with_initial_data(networks);
    let service = NetworkApplicationService::new(repository);

    let query = ListNetworksQuery;

    // Act
    let result = service.list_networks(query).await;

    // Assert
    assert!(result.is_ok());
    let network_dtos = result.unwrap();
    assert_eq!(network_dtos.len(), 2);
    assert!(
        network_dtos
            .iter()
            .any(|n| n.name == Some("Network 1".to_string()))
    );
    assert!(
        network_dtos
            .iter()
            .any(|n| n.name == Some("Network 2".to_string()))
    );
}

#[tokio::test]
async fn test_list_networks_empty() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);

    let query = ListNetworksQuery;

    // Act
    let result = service.list_networks(query).await;

    // Assert
    assert!(result.is_ok());
    let network_dtos = result.unwrap();
    assert!(network_dtos.is_empty());
}

#[tokio::test]
async fn test_delete_network_success() {
    // Arrange
    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let network = Network::new(
        network_id,
        Some("Test Network".to_string()),
        NetworkType::Individual,
        user_id,
        Utc::now(),
    );

    let repository = MockNetworkRepository::with_initial_data(vec![network]);
    let service = NetworkApplicationService::new(repository);

    // Act
    let result = service.delete_network(network_id).await;

    // Assert
    assert!(result.is_ok());

    // Verify network is actually deleted
    let query = GetNetworkQuery { network_id };
    let get_result = service.get_network(query).await.unwrap();
    assert!(get_result.is_none());
}

#[tokio::test]
async fn test_delete_network_not_found() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);

    let non_existent_id = Uuid::new_v4();

    // Act
    let result = service.delete_network(non_existent_id).await;

    // Assert
    assert!(result.is_err());
    match result.unwrap_err() {
        ApplicationError::NotFound => (), // Expected
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_verify_already_verified_network() {
    // Arrange
    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();

    let mut network = Network::new(
        network_id,
        Some("Test Network".to_string()),
        NetworkType::Company,
        user_id,
        Utc::now(),
    );

    // Verify the network first
    network.verify(admin_id).unwrap();

    let repository = MockNetworkRepository::with_initial_data(vec![network]);
    let service = NetworkApplicationService::new(repository);

    let command = VerifyNetworkCommand {
        network_id,
        verified_by: admin_id,
    };

    // Act
    let result = service.verify_network(command).await;

    // Assert
    assert!(result.is_err());
    match result.unwrap_err() {
        ApplicationError::Domain(msg) => {
            assert!(msg.contains("already verified"));
        }
        _ => panic!("Expected Domain error"),
    }
}

#[tokio::test]
async fn test_network_dto_conversion_preserves_all_fields() {
    // Arrange
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

    // Act
    let dto = configurator_service::application::dtos::NetworkDto::from(network);

    // Assert
    assert_eq!(dto.network_id, network_id);
    assert_eq!(dto.name, Some("Test Network".to_string()));
    assert_eq!(dto.network_type, NetworkType::Individual);
    assert_eq!(dto.created_by, user_id);
    assert_eq!(dto.created_at, created_at);
    assert!(!dto.is_verified);
    assert!(dto.is_active);
    assert!(dto.is_live);
    assert!(dto.updated_by.is_none());
    assert!(dto.updated_at.is_none());
}

#[tokio::test]
async fn test_create_multiple_networks_and_list_them() {
    // Arrange
    let repository = MockNetworkRepository::new();
    let service = NetworkApplicationService::new(repository);
    let user_id = Uuid::new_v4();

    // Create multiple networks
    for i in 0..5 {
        let command = CreateNetworkCommand {
            name: Some(format!("Network {}", i)),
            network_type: if i % 2 == 0 {
                NetworkType::Individual
            } else {
                NetworkType::Company
            },
            created_by: user_id,
        };
        service.create_network(command).await.unwrap();
    }

    // Act
    let result = service.list_networks(ListNetworksQuery).await;

    // Assert
    assert!(result.is_ok());
    let networks = result.unwrap();
    assert_eq!(networks.len(), 5);

    // Verify we have both types
    let individual_count = networks
        .iter()
        .filter(|n| n.network_type == NetworkType::Individual)
        .count();
    let company_count = networks
        .iter()
        .filter(|n| n.network_type == NetworkType::Company)
        .count();

    assert_eq!(individual_count, 3); // 0, 2, 4
    assert_eq!(company_count, 2); // 1, 3
}
