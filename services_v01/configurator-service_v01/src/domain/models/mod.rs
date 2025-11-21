//pub mod network;
pub mod person;

// Re-export the main entities for easy access
//pub use network::Network;
pub use person::Person;

// Re-export error types
//pub use network::NetworkError;
pub use person::PersonError;