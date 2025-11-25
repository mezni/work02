use async_trait::async_trait;
use tracing::info;
use crate::domain::events::DomainEvent;
use crate::infrastructure::errors::InfrastructureError;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &DomainEvent) -> Result<(), InfrastructureError>;
}

pub struct EventSubscriber {
    // In a real implementation, this would manage multiple event handlers
    // For different event types
}

impl EventSubscriber {
    pub fn new() -> Self {
        Self
    }

    pub async fn subscribe<T: EventHandler + 'static>(&self, _handler: T) {
        info!("Event handler subscribed");
    }

    pub async fn start(&self) -> Result<(), InfrastructureError> {
        info!("Event subscriber started");
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), InfrastructureError> {
        info!("Event subscriber stopped");
        Ok(())
    }
}