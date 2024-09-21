use chrono::{DateTime, Utc};
use domain::models::clinics::{ClinicModel, CreateClinicDto, CreateDoctorDto, CreatePatientDto, DoctorModel, PatientModel, UpdateClinicDto, UpdateDoctorDto, UpdatePatientDto};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{CLINIC_TABLE, DOCTOR_TABLE, PATIENT_TABLE, SPECIALIZATION_TABLE, USER_TABLE};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorsDb{
    id: Thing,
    user: Thing,
    specialization: Thing,
    clinic: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDoctorDb{
    pub user: Thing,
    pub specialization: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDoctorDb{
    pub user: Thing,
    pub specialization: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientsDb{
    id: Thing,
    user: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePatientDb{
    pub user: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePatientDb{
    pub user: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicsDb{
    id: Thing,
    name: String,
    address: String,
    contact_info: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClinicDb{
    pub name: String,
    pub address: String,
    pub contact_info: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClinicDb{
    pub name: String,
    pub address: String,
    pub contact_info: String,
}

impl From<ClinicModel> for ClinicsDb{
    fn from(value: ClinicModel) -> Self {
        Self { 
            id: Thing::from((CLINIC_TABLE, value.id.as_str())), 
            name: value.name, 
            address: value.address, 
            contact_info: value.contact_info, 
            created_at: value.created_at, 
            updated_at: value.updated_at 
        }
    }
}
impl Into<ClinicModel> for ClinicsDb{
    fn into(self) -> ClinicModel {
        ClinicModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            address: self.address, 
            contact_info: self.contact_info, 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}
impl From<DoctorModel> for DoctorsDb{
    fn from(value: DoctorModel) -> Self {
        Self { 
            id: Thing::from((DOCTOR_TABLE, value.id.as_str())), 
            user: Thing::from((USER_TABLE, value.user.as_str())), 
            specialization: Thing::from((SPECIALIZATION_TABLE, value.specialization.as_str())), 
            clinic: Thing::from((CLINIC_TABLE, value.clinic.as_str())) 
        }
    }
}
impl Into<DoctorModel> for DoctorsDb{
    fn into(self) -> DoctorModel {
        DoctorModel { 
            id: self.id.id.to_string(), 
            user: self.user.id.to_string(), 
            specialization: self.specialization.id.to_string(), 
            clinic: self.clinic.id.to_string() }
    }
}
impl From<PatientModel> for PatientsDb{
    fn from(value: PatientModel) -> Self {
        Self { 
            id: Thing::from((PATIENT_TABLE, value.id.as_str())), 
            user: Thing::from((USER_TABLE, value.user.as_str())) 
        }
    }
}
impl Into<PatientModel> for PatientsDb{
    fn into(self) -> PatientModel {
        PatientModel { 
            id: self.id.id.to_string(),
            user: self.user.id.to_string() 
        }
    }
}
impl Into<CreateDoctorDto> for CreateDoctorDb {
    fn into(self) -> CreateDoctorDto {
        CreateDoctorDto { 
            user: self.user.id.to_string(), 
            specialization: self.specialization.id.to_string() 
        }
    }
}
impl Into<CreatePatientDto> for CreatePatientDb {
    fn into(self) -> CreatePatientDto {
        CreatePatientDto { 
            user: self.user.id.to_string() 
        }
    }
}
impl Into<CreateClinicDto> for CreateClinicDb {
    fn into(self) -> CreateClinicDto {
        CreateClinicDto { 
            name: self.name, 
            address: self.address, 
            contact_info: self.contact_info 
        }
    }
}
impl From<CreateDoctorDto> for CreateDoctorDb {
    fn from(value: CreateDoctorDto) -> Self {
        Self { 
            user: Thing::from((USER_TABLE, value.user.as_str())), 
            specialization: Thing::from((SPECIALIZATION_TABLE, value.specialization.as_str())) }
    }
}
impl From<CreatePatientDto> for CreatePatientDb {
    fn from(value: CreatePatientDto) -> Self {
        Self { 
            user: Thing::from((USER_TABLE, value.user.as_str())) }
    }
}
impl From<CreateClinicDto> for CreateClinicDb {
    fn from(value: CreateClinicDto) -> Self {
        Self { 
            name: value.name, 
            address: value.address, 
            contact_info: value.contact_info 
        }
    }
}

impl Into<UpdateDoctorDto> for UpdateDoctorDb {
    fn into(self) -> UpdateDoctorDto {
        UpdateDoctorDto { 
            user: self.user.id.to_string(), 
            specialization: self.specialization.id.to_string() 
        }
    }
}
impl Into<UpdatePatientDto> for UpdatePatientDb {
    fn into(self) -> UpdatePatientDto {
        UpdatePatientDto { 
            user: self.user.id.to_string() 
        }
    }
}
impl Into<UpdateClinicDto> for UpdateClinicDb {
    fn into(self) -> UpdateClinicDto {
        UpdateClinicDto { 
            name: self.name, 
            address: self.address, 
            contact_info: self.contact_info 
        }
    }
}
impl From<UpdateDoctorDto> for UpdateDoctorDb {
    fn from(value: UpdateDoctorDto) -> Self {
        Self { 
            user: Thing::from((USER_TABLE, value.user.as_str())), 
            specialization: Thing::from((SPECIALIZATION_TABLE, value.specialization.as_str())) }
    }
}
impl From<UpdatePatientDto> for UpdatePatientDb {
    fn from(value: UpdatePatientDto) -> Self {
        Self { 
            user: Thing::from((USER_TABLE, value.user.as_str())) }
    }
}
impl From<UpdateClinicDto> for UpdateClinicDb {
    fn from(value: UpdateClinicDto) -> Self {
        Self { 
            name: value.name, 
            address: value.address, 
            contact_info: value.contact_info 
        }
    }
}



