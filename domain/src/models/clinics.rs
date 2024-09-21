use chrono::{DateTime, Utc};

use super::users::UserModel;

#[derive(Debug, Clone)]
pub struct DoctorModel{
    pub id: String,
    pub user: String,        // TODO пока пусть остаётся так, потом исправим
    pub specialization: String,
    pub clinic: String,
}
#[derive(Debug, Clone)]
pub struct CreateDoctorDto{
    pub user: String,
    pub specialization: String
}
#[derive(Debug, Clone)]
pub struct UpdateDoctorDto{
    pub user: String,
    pub specialization: String
}
#[derive(Debug, Clone)]
pub struct PatientModel{
    pub id: String,
    pub user: String         // TODO пока пусть остаётся так, потом исправим
}
#[derive(Debug, Clone)]
pub struct CreatePatientDto{
    pub user: String
}
#[derive(Debug, Clone)]
pub struct UpdatePatientDto{
    pub user: String
}
#[derive(Debug, Clone)]
pub struct ClinicModel{
    pub id: String,
    pub name: String,
    pub address: String,
    pub contact_info: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone)]
pub struct CreateClinicDto{
    pub name: String,
    pub address: String,
    pub contact_info: String,
}
#[derive(Debug, Clone)]
pub struct UpdateClinicDto{
    pub name: String,
    pub address: String,
    pub contact_info: String,
}