pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure; // ADD THIS LINE

// Re-export commonly used types selectively
pub use domain::{
    enums::network_type::NetworkType,
    events::network_events::{NetworkCreated, NetworkEvent, NetworkVerified},
    models::network::Network,
    repositories::network_repository::{NetworkRepository, RepositoryResult},
};

pub use application::{
    NetworkApplicationService,
    commands::{CreateNetworkCommand, VerifyNetworkCommand},
    dtos::NetworkDto,
    queries::{GetNetworkQuery, ListNetworksQuery},
};

pub use infrastructure::{
    database::{DatabaseConfig, create_pool, create_pool_from_env},
    repositories::PostgresNetworkRepository,
};

pub use api::*; // ADD THIS LINE to export API types
