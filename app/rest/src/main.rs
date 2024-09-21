use std::sync::Arc;
use domain::repositories::database::*;
use domain::services::AppServices;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let database: DatabaseRepo = Arc::new(Box::new(database::pool::DbClient::pool().await?));
    let services = Arc::new(AppServices::init(database));
    let server = api::server::Server::init(services);
    let rst = server.run().await?;
    Ok(())
}
