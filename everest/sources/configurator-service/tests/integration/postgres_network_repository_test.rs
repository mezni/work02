use chrono::Utc;
use configurator_service::{
    domain::enums::network_type::NetworkType,
    domain::models::network::Network,
    domain::repositories::network_repository::NetworkRepository,
    infrastructure::{
        database::{DatabaseConfig, create_pool},
        repositories::PostgresNetworkRepository,
    },
};
use serial_test::serial;
use uuid::Uuid;

// Test helper to create a test database pool
async fn create_test_pool() -> sqlx::PgPool {
    dotenvy::dotenv().ok();

    // Use main database for both development and tests
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://configurator_user:configurator_password@localhost:5432/configurator_db"
            .to_string()
    });

    let config = DatabaseConfig {
        database_url,
        max_connections: 5,
        connect_timeout: std::time::Duration::from_secs(30),
    };

    create_pool(config)
        .await
        .expect("Failed to create test database pool")
}

// Test helper to clean up test data
async fn cleanup_test_data(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM networks WHERE name LIKE 'Test Network%' OR created_at > NOW() - INTERVAL '1 hour'")
        .execute(pool)
        .await
        .expect("Failed to clean up test data");
}

// Skip tests if database is not available
macro_rules! skip_if_no_db {
    () => {
        dotenvy::dotenv().ok();
        if std::env::var("DATABASE_URL").is_err() {
            println!("Skipping database test - no DATABASE_URL set");
            return;
        }
    };
}

// Helper function to create test networks with is_live = false
fn create_test_network(
    network_id: Uuid,
    name: Option<String>,
    network_type: NetworkType,
    created_by: Uuid,
) -> Network {
    let mut network = Network::new(network_id, name, network_type, created_by, Utc::now());
    network.is_live = false;
    network
}

#[tokio::test]
#[serial]
async fn test_save_and_find_network() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let network = create_test_network(
        network_id,
        Some("Test Network - SaveFind".to_string()),
        NetworkType::Company,
        user_id,
    );

    // Test save
    repository
        .save(&network)
        .await
        .expect("Failed to save network");

    // Test find
    let found = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find network");
    assert!(found.is_some());

    let found_network = found.unwrap();
    assert_eq!(found_network.network_id, network_id);
    assert_eq!(
        found_network.name,
        Some("Test Network - SaveFind".to_string())
    );
    assert_eq!(found_network.network_type, NetworkType::Company);
    assert_eq!(found_network.created_by, user_id);
    assert!(!found_network.is_verified);
    assert!(found_network.is_active);
    assert!(!found_network.is_live); // Expect false

    cleanup_test_data(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_save_updates_existing_network() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let mut network = create_test_network(
        network_id,
        Some("Original Name".to_string()),
        NetworkType::Individual,
        user_id,
    );

    // Save original
    repository
        .save(&network)
        .await
        .expect("Failed to save original network");

    // Update and save again
    network.name = Some("Updated Name".to_string());
    network.network_type = NetworkType::Company;
    // is_live remains false
    repository
        .save(&network)
        .await
        .expect("Failed to update network");

    // Verify update
    let found = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find updated network");
    assert!(found.is_some());

    let found_network = found.unwrap();
    assert_eq!(found_network.name, Some("Updated Name".to_string()));
    assert_eq!(found_network.network_type, NetworkType::Company);
    assert!(!found_network.is_live); // Should still be false

    cleanup_test_data(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_delete_network() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let network = create_test_network(
        network_id,
        Some("Test Network - Delete".to_string()),
        NetworkType::Individual,
        user_id,
    );

    // Save then delete
    repository
        .save(&network)
        .await
        .expect("Failed to save network");

    // Verify it exists
    let found_before = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find network before delete");
    assert!(found_before.is_some());

    // Delete
    repository
        .delete(network_id)
        .await
        .expect("Failed to delete network");

    // Verify it's gone
    let found_after = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find network after delete");
    assert!(found_after.is_none());

    cleanup_test_data(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_list_networks() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let user_id = Uuid::new_v4();

    // Create multiple networks
    let networks = vec![
        create_test_network(
            Uuid::new_v4(),
            Some("Test Network - List 1".to_string()),
            NetworkType::Individual,
            user_id,
        ),
        create_test_network(
            Uuid::new_v4(),
            Some("Test Network - List 2".to_string()),
            NetworkType::Company,
            user_id,
        ),
        create_test_network(
            Uuid::new_v4(),
            Some("Test Network - List 3".to_string()),
            NetworkType::Individual,
            user_id,
        ),
    ];

    // Save all networks
    for network in &networks {
        repository
            .save(network)
            .await
            .expect("Failed to save network");
    }

    // Test list
    let all_networks = repository.list().await.expect("Failed to list networks");

    // Filter for our test networks
    let test_networks: Vec<_> = all_networks
        .into_iter()
        .filter(|n| {
            n.name
                .as_ref()
                .map(|name| name.starts_with("Test Network - List"))
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(test_networks.len(), 3);

    // Verify we have both types and all have is_live = false
    let individual_count = test_networks
        .iter()
        .filter(|n| n.network_type == NetworkType::Individual)
        .count();
    let company_count = test_networks
        .iter()
        .filter(|n| n.network_type == NetworkType::Company)
        .count();

    assert_eq!(individual_count, 2);
    assert_eq!(company_count, 1);

    // Verify all test networks have is_live = false
    for network in &test_networks {
        assert!(!network.is_live);
    }

    cleanup_test_data(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_find_non_existent_network() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    let non_existent_id = Uuid::new_v4();

    let found = repository
        .find_by_id(non_existent_id)
        .await
        .expect("Failed to query non-existent network");
    assert!(found.is_none());
}

#[tokio::test]
#[serial]
async fn test_network_with_verification() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();

    let mut network = create_test_network(
        network_id,
        Some("Test Network - Verified".to_string()),
        NetworkType::Company,
        user_id,
    );

    // Save unverified network
    repository
        .save(&network)
        .await
        .expect("Failed to save unverified network");

    // Verify the network
    network.verify(admin_id).expect("Failed to verify network");
    repository
        .save(&network)
        .await
        .expect("Failed to save verified network");

    // Retrieve and verify
    let found = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find verified network");
    assert!(found.is_some());

    let found_network = found.unwrap();
    assert!(found_network.is_verified);
    assert_eq!(found_network.updated_by, Some(admin_id));
    assert!(found_network.updated_at.is_some());
    assert!(!found_network.is_live); // Should still be false

    cleanup_test_data(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_network_without_name() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let network = create_test_network(
        network_id,
        None, // No name
        NetworkType::Individual,
        user_id,
    );

    repository
        .save(&network)
        .await
        .expect("Failed to save network without name");

    let found = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find network without name");
    assert!(found.is_some());

    let found_network = found.unwrap();
    assert!(found_network.name.is_none());
    assert!(!found_network.is_live); // Should be false

    cleanup_test_data(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_multiple_operations_on_same_network() {
    skip_if_no_db!();

    let pool = create_test_pool().await;
    let repository = PostgresNetworkRepository::new(pool.clone());

    cleanup_test_data(&pool).await;

    let network_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let admin_id = Uuid::new_v4();

    // Create network
    let mut network = create_test_network(
        network_id,
        Some("Test Network - Multiple Ops".to_string()),
        NetworkType::Company,
        user_id,
    );

    // Save
    repository
        .save(&network)
        .await
        .expect("Failed to save network");

    // Find and verify
    let found = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find network");
    assert!(found.is_some());
    assert!(!found.unwrap().is_live);

    // Update
    network.name = Some("Updated Name".to_string());
    repository
        .save(&network)
        .await
        .expect("Failed to update network");

    // Verify and save again
    network.verify(admin_id).expect("Failed to verify network");
    repository
        .save(&network)
        .await
        .expect("Failed to save verified network");

    // Final check
    let final_network = repository
        .find_by_id(network_id)
        .await
        .expect("Failed to find final network")
        .unwrap();
    assert_eq!(final_network.name, Some("Updated Name".to_string()));
    assert!(final_network.is_verified);
    assert!(!final_network.is_live); // Should still be false

    cleanup_test_data(&pool).await;
}
