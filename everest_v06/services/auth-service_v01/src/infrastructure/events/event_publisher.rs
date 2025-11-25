use async_trait::async_trait;
use tracing::{info, error};
use crate::domain::events::{DomainEvent, Event};
use crate::infrastructure::errors::InfrastructureError;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &DomainEvent) -> Result<(), InfrastructureError>;
    async fn publish_batch(&self, events: &[DomainEvent]) -> Result<(), InfrastructureError>;
}

pub struct NoOpEventPublisher;

#[async_trait]
impl EventPublisher for NoOpEventPublisher {
    async fn publish(&self, _event: &DomainEvent) -> Result<(), InfrastructureError> {
        info!("No-op event publisher: event would be published here");
        Ok(())
    }

    async fn publish_batch(&self, events: &[DomainEvent]) -> Result<(), InfrastructureError> {
        info!("No-op event publisher: {} events would be published here", events.len());
        Ok(())
    }
}