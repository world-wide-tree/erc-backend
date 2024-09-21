use chrono::{DateTime, Utc};

use super::users::UserModel;

#[derive(Debug, Clone)]
pub struct DoctorModel{
    pub id: String,
    pub user: UserModel,        // TODO пока пусть остаётся так, потом исправим
    pub specialization: String,
    pub clinic: String,
}
#[derive(Debug, Clone)]
pub struct PatientModel{
    pub id: String,
    pub user: UserModel         // TODO пока пусть остаётся так, потом исправим
}

#[derive(Debug, Clone)]
pub struct ClinicModel{
    pub id: String,
    pub name: String,
    pub address: String,
    pub contact_info: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
