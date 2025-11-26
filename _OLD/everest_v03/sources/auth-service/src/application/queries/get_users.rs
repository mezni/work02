// src/application/queries/get_users.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetUsersQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    
    #[serde(default = "default_active_only")]
    pub active_only: bool,
}

fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }
fn default_active_only() -> bool { true }

impl GetUsersQuery {
    pub fn new(page: Option<u32>, page_size: Option<u32>, active_only: Option<bool>) -> Self {
        Self {
            page: page.unwrap_or_else(default_page),
            page_size: page_size.unwrap_or_else(default_page_size),
            active_only: active_only.unwrap_or_else(default_active_only),
        }
    }
    
    pub fn calculate_offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }
}