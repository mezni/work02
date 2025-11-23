// src/domain/value_objects/mod.rs
pub mod email;
pub mod username;
pub mod user_id;
pub mod phone_number;

pub use email::Email;
pub use username::Username;
pub use user_id::UserId;
pub use phone_number::PhoneNumber;