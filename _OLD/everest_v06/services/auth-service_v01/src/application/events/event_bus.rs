use async_trait::async_trait;
use tracing::{info, error};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::domain::events::{DomainEvent, Event};
use crate::infrastructure::events::event_publisher::EventPublisher;
use crate::application::errors::ApplicationError;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &DomainEvent) -> Result<(), ApplicationError>;
}

pub struct EventBus {
    publisher: Arc<dyn EventPublisher>,
    tx: broadcast::Sender<DomainEvent>,
}

impl EventBus {
    pub fn new(publisher: Arc<dyn EventPublisher>) -> Self {
        let (tx, _) = broadcast::channel(100);
        
        Self {
            publisher,
            tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.tx.subscribe()
    }

    pub async fn publish(&self, event: DomainEvent) -> Result<(), ApplicationError> {
        info!("Publishing event: {}", event.event_type);
        
        // Publish to external systems
        if let Err(e) = self.publisher.publish(&event).await {
            error!("Failed to publish event externally: {}", e);
            // Don't fail the operation if external publishing fails
        }
        
        // Publish internally
        let _ = self.tx.send(event);
        
        Ok(())
    }

    pub async fn publish_many(&self, events: Vec<DomainEvent>) -> Result<(), ApplicationError> {
        info!("Publishing {} events", events.len());
        
        // Publish to external systems
        if let Err(e) = self.publisher.publish_batch(&events).await {
            error!("Failed to publish events externally: {}", e);
        }
        
        // Publish internally
        for event in events {
            let _ = self.tx.send(event);
        }
        
        Ok(())
    }
}

pub struct EventProcessor {
    event_bus: Arc<EventBus>,
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventProcessor {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            event_bus,
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub async fn start(&self) -> Result<(), ApplicationError> {
        let mut receiver = self.event_bus.subscribe();
        let handlers = self.handlers.clone();
        
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                for handler in &handlers {
                    if let Err(e) = handler.handle(&event).await {
                        error!("Error handling event {}: {}", event.event_type, e);
                    }
                }
            }
        });
        
        info!("Event processor started with {} handlers", self.handlers.len());
        Ok(())
    }
}