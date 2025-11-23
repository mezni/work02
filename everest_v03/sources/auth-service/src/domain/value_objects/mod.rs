// src/domain/value_objects/mod.rs
pub mod email;
pub mod phone_number;
pub mod user_id;
pub mod username;

pub use email::Email;
pub use phone_number::PhoneNumber;
pub use user_id::UserId;
pub use username::Username;
