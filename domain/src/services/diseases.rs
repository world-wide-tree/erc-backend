use anyhow::Result;

use crate::{models::diseases::{CreateDieaseasDto, DieseasModel, UpdateDieaseasDto}, repositories::database::DatabaseRepo};

pub struct DieseasService{
    db: DatabaseRepo
}

impl DieseasService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(&self, dto: CreateDieaseasDto) -> Result<DieseasModel>{
        self.db.create_diseases(dto).await        
    }
    pub async fn get_by_id(&self, id: String) -> Result<DieseasModel>{
        self.db.get_diseases(id).await 
    }
    pub async fn list(&self) -> Result<Vec<DieseasModel>>{
        self.db.list_diseases().await 
    }
    pub async fn update(&self, id: String, dto: UpdateDieaseasDto ) -> Result<DieseasModel>{
        self.db.update_diseases(id, dto).await 
    }
    pub async fn delete(&self, id: String) -> Result<()>{
        self.db.delete_diseases(id).await 
    }
}
