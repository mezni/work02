use super::types::{AuditInfo, OrganizationId, StationId, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StationStatus {
    Active,
    Inactive,
    Maintenance,
}

#[derive(Debug, Clone)]
pub struct Station {
    pub id: StationId,
    pub name: String,
    pub location: Option<String>, // Could be enhanced with proper Location type
    pub organization_id: OrganizationId,
    pub status: StationStatus,
    pub audit: AuditInfo,
}

impl Station {
    pub fn new(
        name: String,
        organization_id: OrganizationId,
        created_by: UserId,
    ) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Station name cannot be empty".to_string());
        }

        Ok(Self {
            id: StationId::new_v4(),
            name: name.trim().to_string(),
            location: None,
            organization_id,
            status: StationStatus::Active,
            audit: AuditInfo::new(created_by),
        })
    }
}
