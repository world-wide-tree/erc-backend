use crate::repositories::database::DatabaseRepo;

pub struct ErcService{
    db: DatabaseRepo
}

impl ErcService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(){}
    pub async fn get_by_id(id: String){}
    pub async fn list(){}
    pub async fn update(){}
    pub fn delete(){}
}
