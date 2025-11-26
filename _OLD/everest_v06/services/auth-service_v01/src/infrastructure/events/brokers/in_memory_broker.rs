use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tracing::{info, error};
use crate::domain::events::DomainEvent;
use crate::infrastructure::events::event_publisher::EventPublisher;
use crate::infrastructure::errors::InfrastructureError;

#[derive(Clone)]
pub struct InMemoryEventBroker {
    events: Arc<Mutex<VecDeque<DomainEvent>>>,
    max_capacity: usize,
}

impl InMemoryEventBroker {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::with_capacity(max_capacity))),
            max_capacity,
        }
    }

    pub fn pop_event(&self) -> Option<DomainEvent> {
        let mut events = self.events.lock().unwrap();
        events.pop_front()
    }

    pub fn event_count(&self) -> usize {
        let events = self.events.lock().unwrap();
        events.len()
    }

    pub fn is_empty(&self) -> bool {
        let events = self.events.lock().unwrap();
        events.is_empty()
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventBroker {
    async fn publish(&self, event: &DomainEvent) -> Result<(), InfrastructureError> {
        let mut events = self.events.lock().unwrap();
        
        if events.len() >= self.max_capacity {
            error!("Event broker capacity exceeded");
            return Err(InfrastructureError::EventBus("Event broker capacity exceeded".to_string()));
        }

        events.push_back(event.clone());
        info!("Event published to in-memory broker: {}", event.event_type);
        
        Ok(())
    }

    async fn publish_batch(&self, events: &[DomainEvent]) -> Result<(), InfrastructureError> {
        let mut current_events = self.events.lock().unwrap();
        
        if current_events.len() + events.len() > self.max_capacity {
            error!("Event broker capacity exceeded for batch publish");
            return Err(InfrastructureError::EventBus("Event broker capacity exceeded".to_string()));
        }

        for event in events {
            current_events.push_back(event.clone());
        }
        
        info!("{} events published to in-memory broker", events.len());
        Ok(())
    }
}