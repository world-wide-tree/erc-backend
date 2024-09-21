use anyhow::Result;

use crate::{models::erc::{CreateErcDto, ErcModel, UpdateErcDto}, repositories::database::DatabaseRepo};

pub struct ErcService{
    db: DatabaseRepo
}

impl ErcService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(&self, dto: CreateErcDto) -> Result<ErcModel>{
        self.db.create_erc(dto).await        
    }
    pub async fn get_by_id(&self, id: String) -> Result<ErcModel>{
        self.db.get_erc(id).await 
    }
    pub async fn list(&self) -> Result<Vec<ErcModel>>{
        self.db.list_erc().await 
    }
    pub async fn update(&self, id: String, dto: UpdateErcDto ) -> Result<ErcModel>{
        self.db.update_erc(id, dto).await 
    }
    pub async fn delete(&self, id: String) -> Result<()>{
        self.db.delete_erc(id).await 
    }
}
