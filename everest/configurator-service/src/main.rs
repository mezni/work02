use configurator_service::run;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    run().await
}
