use crate::application::commands::{CreateNetworkCommand, VerifyNetworkCommand};
use crate::application::dtos::NetworkDto;
use crate::application::queries::{GetNetworkQuery, ListNetworksQuery};
use crate::domain::models::network::Network;
use crate::domain::repositories::network_repository::NetworkRepository;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Network not found")]
    NotFound,
    #[error("Network already verified")]
    AlreadyVerified,
    #[error("Repository error: {0}")]
    Repository(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Domain error: {0}")]
    Domain(String),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;

pub struct NetworkApplicationService<T: NetworkRepository> {
    network_repository: T,
}

impl<T: NetworkRepository> NetworkApplicationService<T> {
    pub fn new(network_repository: T) -> Self {
        Self { network_repository }
    }

    // Command handlers
    pub async fn create_network(
        &self,
        command: CreateNetworkCommand,
    ) -> ApplicationResult<NetworkDto> {
        let network = Network::new(
            Uuid::new_v4(),
            command.name,
            command.network_type,
            command.created_by,
            chrono::Utc::now(),
        );

        self.network_repository
            .save(&network)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(NetworkDto::from(network))
    }

    pub async fn verify_network(
        &self,
        command: VerifyNetworkCommand,
    ) -> ApplicationResult<NetworkDto> {
        let mut network = self
            .network_repository
            .find_by_id(command.network_id)
            .await
            .map_err(ApplicationError::Repository)?
            .ok_or(ApplicationError::NotFound)?;

        network
            .verify(command.verified_by)
            .map_err(|e| ApplicationError::Domain(e))?;

        self.network_repository
            .save(&network)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(NetworkDto::from(network))
    }

    // Query handlers
    pub async fn get_network(
        &self,
        query: GetNetworkQuery,
    ) -> ApplicationResult<Option<NetworkDto>> {
        let network = self
            .network_repository
            .find_by_id(query.network_id)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(network.map(NetworkDto::from))
    }

    pub async fn list_networks(
        &self,
        _query: ListNetworksQuery,
    ) -> ApplicationResult<Vec<NetworkDto>> {
        let networks = self
            .network_repository
            .list()
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(networks.into_iter().map(NetworkDto::from).collect())
    }

    pub async fn delete_network(&self, network_id: Uuid) -> ApplicationResult<()> {
        // Check if network exists
        let exists = self
            .network_repository
            .find_by_id(network_id)
            .await
            .map_err(ApplicationError::Repository)?
            .is_some();

        if !exists {
            return Err(ApplicationError::NotFound);
        }

        self.network_repository
            .delete(network_id)
            .await
            .map_err(ApplicationError::Repository)?;

        Ok(())
    }
}
