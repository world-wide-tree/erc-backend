use anyhow::Result;
use dotenv::dotenv;
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

pub struct DbClient{
    pool: Surreal<Client>,
}
impl DbClient{
    pub async fn pool() -> Result<Self>{
        dotenv().ok();

        let db_location = "127.0.0.1:7000";
        let db_namespace = "Erc";
        let db_database = "Erc";
        let db_user = "root";
        let db_password = "root";

        let db = Surreal::new::<Ws>(db_location).await?;

        db.signin(Root{
            username: db_user,
            password: db_password
        })
        .await?;

        db.use_ns(db_namespace).use_db(db_database).await?;
        Ok(Self { pool: db })
    }
}