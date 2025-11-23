use async_trait::async_trait;
use sqlx::{Error, PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::enums::network_type::NetworkType;
use crate::domain::models::network::Network;
use crate::domain::repositories::network_repository::{NetworkRepository, RepositoryResult};

#[derive(Debug, Clone)]
pub struct PostgresNetworkRepository {
    pool: PgPool,
}

impl PostgresNetworkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NetworkRepository for PostgresNetworkRepository {
    async fn save(&self, network: &Network) -> RepositoryResult<()> {
        let query = r#"
            INSERT INTO networks (
                network_id, name, network_type, is_verified, is_active, is_live,
                created_by, updated_by, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (network_id) 
            DO UPDATE SET 
                name = $2, network_type = $3, is_verified = $4, is_active = $5, 
                is_live = $6, updated_by = $8, updated_at = $10
        "#;

        sqlx::query(query)
            .bind(network.network_id)
            .bind(&network.name)
            .bind(network.network_type.to_string())
            .bind(network.is_verified)
            .bind(network.is_active)
            .bind(network.is_live)
            .bind(network.created_by)
            .bind(network.updated_by)
            .bind(network.created_at)
            .bind(network.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    async fn find_by_id(&self, network_id: Uuid) -> RepositoryResult<Option<Network>> {
        let query = r#"
            SELECT 
                network_id, name, network_type, is_verified, is_active, is_live,
                created_by, updated_by, created_at, updated_at
            FROM networks 
            WHERE network_id = $1
        "#;

        let result = sqlx::query(query)
            .bind(network_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        match result {
            Some(row) => {
                let network_type_str: String = row.get("network_type");
                let network_type = NetworkType::from_str(&network_type_str).map_err(|e| {
                    Box::new(Error::Decode(e.into())) as Box<dyn std::error::Error + Send + Sync>
                })?;

                Ok(Some(Network {
                    network_id: row.get("network_id"),
                    name: row.get("name"),
                    network_type,
                    is_verified: row.get("is_verified"),
                    is_active: row.get("is_active"),
                    is_live: row.get("is_live"),
                    created_by: row.get("created_by"),
                    updated_by: row.get("updated_by"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    events: vec![], // Add empty events vector
                }))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, network_id: Uuid) -> RepositoryResult<()> {
        let query = "DELETE FROM networks WHERE network_id = $1";

        sqlx::query(query)
            .bind(network_id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    async fn list(&self) -> RepositoryResult<Vec<Network>> {
        let query = r#"
            SELECT 
                network_id, name, network_type, is_verified, is_active, is_live,
                created_by, updated_by, created_at, updated_at
            FROM networks 
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let mut networks = Vec::new();

        for row in rows {
            let network_type_str: String = row.get("network_type");
            let network_type = NetworkType::from_str(&network_type_str).map_err(|e| {
                Box::new(Error::Decode(e.into())) as Box<dyn std::error::Error + Send + Sync>
            })?;

            networks.push(Network {
                network_id: row.get("network_id"),
                name: row.get("name"),
                network_type,
                is_verified: row.get("is_verified"),
                is_active: row.get("is_active"),
                is_live: row.get("is_live"),
                created_by: row.get("created_by"),
                updated_by: row.get("updated_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                events: vec![], // Add empty events vector
            });
        }

        Ok(networks)
    }
}
