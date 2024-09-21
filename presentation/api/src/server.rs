use std::sync::Arc;

use anyhow::Result;
use domain::services::AppServices;
use dotenv::dotenv;

use crate::controllers::app_routes;

pub struct Server{
    port: u16,
    state: Arc<AppServices>
}

impl Server{
    pub fn init(state: Arc<AppServices>) -> Self{
        dotenv().ok();
        Self { port: 3000, state }
    }
    pub async fn run(self) -> Result<()>{
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app_routes(self.state)).await?;
        Ok(())
    }
}