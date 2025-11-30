use crate::domain::station::Station;
use crate::errors::AppError;
use uuid::Uuid;

#[derive(Clone)]
pub struct StationService;

impl StationService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_station(
        &self,
        name: String,
        org_id: Option<Uuid>,
    ) -> Result<Station, AppError> {
        Ok(Station::new(name, org_id))
    }
}
