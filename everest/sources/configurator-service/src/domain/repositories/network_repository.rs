use crate::domain::models::network::Network;
use async_trait::async_trait;
use uuid::Uuid;

/// Custom result type for repository operations
pub type RepositoryResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Repository trait for `Network` aggregate
#[async_trait]
pub trait NetworkRepository: Send + Sync {
    /// Save a new network or update an existing one
    async fn save(&self, network: &Network) -> RepositoryResult<()>;

    /// Find a network by ID
    async fn find_by_id(&self, network_id: Uuid) -> RepositoryResult<Option<Network>>;

    /// Delete a network by ID
    async fn delete(&self, network_id: Uuid) -> RepositoryResult<()>;

    /// Check if a network exists by ID
    async fn exists(&self, network_id: Uuid) -> RepositoryResult<bool> {
        Ok(self.find_by_id(network_id).await?.is_some())
    }

    /// List all networks
    async fn list(&self) -> RepositoryResult<Vec<Network>>;
}
