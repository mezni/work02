// src/application/mod.rs
pub mod services;
pub mod commands;
pub mod queries;
pub mod dtos;
pub mod error;

pub use services::{UserService, UserApplicationService};
pub use commands::{CreateUserCommand, UpdateUserCommand, DeactivateUserCommand};
pub use queries::{GetUsersQuery, GetUserByIdQuery, SearchUsersQuery};
pub use dtos::{UserDto, UserListDto};
pub use error::ApplicationError;