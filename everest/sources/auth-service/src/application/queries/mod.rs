// src/application/queries/mod.rs
pub mod get_users;
pub mod get_user_by_id;
pub mod search_users;

pub use get_users::GetUsersQuery;
pub use get_user_by_id::GetUserByIdQuery;
pub use search_users::SearchUsersQuery;