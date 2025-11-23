use uuid::Uuid;

#[derive(Debug)]
pub struct GetNetworkQuery {
    pub network_id: Uuid,
}

impl GetNetworkQuery {
    pub fn new(network_id: Uuid) -> Self {
        Self { network_id }
    }
}
