use crate::repositories::database::DatabaseRepo;

pub struct DieseasService{
    db: DatabaseRepo
}

impl DieseasService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(){}
    pub async fn get_by_id(id: String){}
    pub async fn list(){}
    pub async fn update(){}
    pub fn delete(){}
}
