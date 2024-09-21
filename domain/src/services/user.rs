use anyhow::Result;

use crate::{models::users::{CreateUserDto, UpdateUserDto, UserModel}, repositories::database::DatabaseRepo};

pub struct UserService{
    db: DatabaseRepo
}

impl UserService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(&self, dto: CreateUserDto) -> Result<UserModel>{
        self.db.create_user(dto).await        
    }
    pub async fn get_by_id(&self, id: String) -> Result<UserModel>{
        self.db.get_user(id).await 
    }
    pub async fn list(&self) -> Result<Vec<UserModel>>{
        self.db.list_user().await 
    }
    pub async fn update(&self, id: String, dto: UpdateUserDto ) -> Result<UserModel>{
        self.db.update_user(id, dto).await 
    }
    pub async fn delete(&self, id: String) -> Result<()>{
        self.db.delete_user(id).await 
    }
}
