use anyhow::Result;
use dotenv::dotenv;

use crate::controllers::app_routes;

pub struct Server{
    port: u16,
}

impl Server{
    pub fn init() -> Self{
        dotenv().ok();
        Self { port: 3000 }
    }
    pub async fn run(self) -> Result<()>{
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app_routes()).await?;
        Ok(())
    }
}