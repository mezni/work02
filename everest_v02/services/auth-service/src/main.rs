use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    auth_service::run().await
}
