use crate::repositories::database::DatabaseRepo;

pub struct UserService{
    db: DatabaseRepo
}

impl UserService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn signin(){}
    pub async fn get_by_id(id: String){}
    pub async fn list(){}
    pub async fn update(){}
    pub fn delete(){}
}
