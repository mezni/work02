use super::dtos::{AppInfoResponse, UserResponse};
use super::{commands, queries};
use crate::error::Result;

pub struct AppService;

impl AppService {
    pub fn get_app_info(&self) -> Result<AppInfoResponse> {
        let query = queries::GetAppInfoQuery;
        query.execute()
    }

    pub fn update_app_name(&self, new_name: String) -> Result<String> {
        let command = commands::UpdateAppNameCommand { new_name };
        command.execute()
    }

    pub fn get_user(&self, user_id: u64) -> Result<UserResponse> {
        let query = queries::GetUserQuery { user_id };
        query.execute()
    }

    pub fn create_user(&self, username: String, email: String) -> Result<UserResponse> {
        let command = commands::CreateUserCommand { username, email };
        command.execute()
    }
}
