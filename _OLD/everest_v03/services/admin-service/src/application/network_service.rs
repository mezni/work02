impl NetworkService {
    pub async fn create_network(
        &self,
        request: CreateNetworkRequest,
        claims: &Claims,
    ) -> Result<Network, DomainError> {
        // Check if user is ADMIN
        if !claims.is_admin() {
            return Err(DomainError::Unauthorized(
                "Only admins can create networks".to_string()
            ));
        }

        let network_id = generate_network_id();
        let created_by = &claims.sub;

        self.network_repo.create(network_id, request, created_by).await
    }

    pub async fn list_networks(
        &self,
        claims: &Claims,
    ) -> Result<Vec<Network>, DomainError> {
        if claims.is_admin() {
            // Admin: see all networks
            self.network_repo.list_all().await
        } else if claims.is_partner() {
            // Partner: see their network
            let network_id = claims.get_network_id()
                .ok_or_else(|| DomainError::Unauthorized("No network_id in token".to_string()))?;
            self.network_repo.list_by_network_id(&network_id).await
        } else {
            Err(DomainError::Unauthorized("Insufficient permissions".to_string()))
        }
    }
}