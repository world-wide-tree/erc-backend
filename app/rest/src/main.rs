#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let database = database::pool::DbClient::pool().await?;
    let server = api::server::Server::init();
    let rst = server.run().await?;
    Ok(())
}
