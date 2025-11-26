// src/application/dtos/user_list_dto.rs
use serde::Serialize;
use crate::application::dtos::UserDto;

#[derive(Debug, Serialize)]
pub struct UserListDto {
    pub users: Vec<UserDto>,
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl UserListDto {
    pub fn new(
        users: Vec<UserDto>,
        page: u32,
        page_size: u32,
        total_count: u64,
    ) -> Self {
        let total_pages = ((total_count as f64) / (page_size as f64)).ceil() as u32;
        let has_next = page < total_pages;
        let has_prev = page > 1;
        
        Self {
            users,
            page,
            page_size,
            total_count,
            total_pages,
            has_next,
            has_prev,
        }
    }
    
    pub fn empty(page: u32, page_size: u32) -> Self {
        Self::new(Vec::new(), page, page_size, 0)
    }
}