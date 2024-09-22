use chrono::{DateTime, Utc};
use domain::models::clinics::{ClinicModel, CreateClinicDto, CreateDoctorDto, CreatePatientDto, DoctorModel, PatientModel, UpdateClinicDto, UpdateDoctorDto, UpdatePatientDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DoctorApiDto{
    pub id: String,
    pub user: String,
    pub specialization: String,
    pub clinic: String
}
impl From<DoctorModel> for DoctorApiDto{
    fn from(value: DoctorModel) -> Self {
        Self { 
            id: value.id, 
            user: value.user, 
            specialization: value.specialization, 
            clinic: value.clinic 
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PatientApiDto{
    pub id: String,
    pub user: String        
}
impl From<PatientModel> for PatientApiDto{
    fn from(value: PatientModel) -> Self {
        Self { 
            id: value.id, 
            user: value.user 
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClinicApiDto{
    pub id: String,
    pub name: String,
    pub address: String,
    pub contact_info: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl From<ClinicModel> for ClinicApiDto{
    fn from(value: ClinicModel) -> Self {
        ClinicApiDto { 
            id: value.id, 
            name: value.name, 
            address: value.address, 
            contact_info: value.contact_info, 
            created_at: value.created_at, 
            updated_at: value.updated_at 
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostDoctorApiDto{
    pub user: String,
    pub specialization: String
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostPatientApiDto{
    pub user: String
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostClinicApiDto{
    pub name: String,
    pub address: String,
    pub contact_info: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PutDoctorApiDto{
    pub user: String,
    pub specialization: String
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PutPatientApiDto{
    pub user: String
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PutClinicApiDto{
    pub name: String,
    pub address: String,
    pub contact_info: String,
}

impl Into<DoctorModel> for DoctorApiDto{
    fn into(self) -> DoctorModel {
        DoctorModel { 
            id: self.id, 
            user: self.user, 
            specialization: self.specialization, 
            clinic: self.clinic 
        }
    }
}
impl Into<CreateDoctorDto> for PostDoctorApiDto{
    fn into(self) -> CreateDoctorDto {
        CreateDoctorDto { 
            user: self.user, 
            specialization: self.specialization 
        }
    }
}
impl Into<UpdateDoctorDto> for PutDoctorApiDto{
    fn into(self) -> UpdateDoctorDto {
        UpdateDoctorDto { 
            user: self.user, 
            specialization: self.specialization 
        }
    }
}
impl Into<PatientModel> for PatientApiDto{
    fn into(self) -> PatientModel {
        PatientModel { 
            id: self.id, 
            user: self.user 
        }
    }
}
impl Into<CreatePatientDto> for PostPatientApiDto{
    fn into(self) -> CreatePatientDto {
        CreatePatientDto { 
            user: self.user 
        }
    }
}
impl Into<UpdatePatientDto> for PutPatientApiDto{
    fn into(self) -> UpdatePatientDto {
        UpdatePatientDto { user: self.user }
    }
}
impl Into<ClinicModel> for ClinicApiDto{
    fn into(self) -> ClinicModel {
        ClinicModel { 
            id: self.id, 
            name: self.name, 
            address: self.address, 
            contact_info: self.contact_info, 
            created_at: self.created_at, 
            updated_at: self.updated_at
        }
    }
}
impl Into<CreateClinicDto> for PostClinicApiDto{
    fn into(self) -> CreateClinicDto {
        CreateClinicDto { 
            name: self.name, 
            address: self.address, 
            contact_info: self.contact_info 
        }
    }
}
impl Into<UpdateClinicDto> for PutClinicApiDto{
    fn into(self) -> UpdateClinicDto {
        UpdateClinicDto { 
            name: self.name, 
            address: self.address, 
            contact_info: self.contact_info 
        }
    }
}