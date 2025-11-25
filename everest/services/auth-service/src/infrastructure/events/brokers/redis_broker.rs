use async_trait::async_trait;
use redis::{Client, Commands, RedisError};
use tracing::{info, error};
use crate::domain::events::DomainEvent;
use crate::infrastructure::events::event_publisher::EventPublisher;
use crate::infrastructure::errors::InfrastructureError;

pub struct RedisEventBroker {
    client: Client,
    channel: String,
}

impl RedisEventBroker {
    pub fn new(redis_url: &str, channel: &str) -> Result<Self, InfrastructureError> {
        let client = Client::open(redis_url)
            .map_err(|e| {
                error!("Failed to create Redis client: {}", e);
                InfrastructureError::EventBus(format!("Redis client creation failed: {}", e))
            })?;

        Ok(Self {
            client,
            channel: channel.to_string(),
        })
    }

    pub async fn connect(&self) -> Result<redis::Connection, InfrastructureError> {
        self.client.get_connection()
            .map_err(|e| {
                error!("Failed to connect to Redis: {}", e);
                InfrastructureError::EventBus(format!("Redis connection failed: {}", e))
            })
    }
}

#[async_trait]
impl EventPublisher for RedisEventBroker {
    async fn publish(&self, event: &DomainEvent) -> Result<(), InfrastructureError> {
        let mut conn = self.connect().await?;
        let event_data = serde_json::to_string(event)
            .map_err(|e| InfrastructureError::Serialization(e))?;

        conn.publish(&self.channel, event_data)
            .map_err(|e: RedisError| {
                error!("Failed to publish event to Redis: {}", e);
                InfrastructureError::EventBus(format!("Redis publish failed: {}", e))
            })?;

        info!("Event published to Redis: {}", event.event_type);
        Ok(())
    }

    async fn publish_batch(&self, events: &[DomainEvent]) -> Result<(), InfrastructureError> {
        let mut conn = self.connect().await?;
        
        for event in events {
            let event_data = serde_json::to_string(event)
                .map_err(|e| InfrastructureError::Serialization(e))?;

            conn.publish(&self.channel, event_data)
                .map_err(|e: RedisError| {
                    error!("Failed to publish event to Redis: {}", e);
                    InfrastructureError::EventBus(format!("Redis publish failed: {}", e))
                })?;
        }

        info!("{} events published to Redis", events.len());
        Ok(())
    }
}