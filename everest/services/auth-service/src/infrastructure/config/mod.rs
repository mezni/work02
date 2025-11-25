// src/infrastructure/config/mod.rs
pub mod settings;

// Re-export the actual structs from your settings.rs
pub use settings::{
    AuditSettings, AuthSettings, CacheSettings, DatabaseSettings, KeycloakSettings, ServerSettings,
    Settings,
};
