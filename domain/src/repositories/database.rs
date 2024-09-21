use std::sync::Arc;

use anyhow::Result;

use crate::models::{clinics::{ClinicModel, CreateClinicDto, CreateDoctorDto, CreatePatientDto, DoctorModel, PatientModel, UpdateClinicDto, UpdateDoctorDto, UpdatePatientDto}, diseases::{CreateDieaseasDto, CreateSpecializationDto, DieseasModel, SpecializationModel, UpdateDieaseasDto, UpdateSpecializationDto}, erc::{CreateErcDto, ErcModel, UpdateErcDto}, users::{CreateUserDto, UpdateUserDto, UserModel}};


pub type DatabaseRepo = Arc<Box<dyn DatabaseRepositories>>;


#[async_trait::async_trait]
pub trait DatabaseRepositories:
    DbClinicsRepositories + 
    DbDiseasesRepositories + 
    DbErcRepositories + 
    DbUsersRepositories
{}

#[async_trait::async_trait]
pub trait DbClinicsRepositories{
    async fn create_clinic(&self, dto: CreateClinicDto) -> Result<ClinicModel>;
    async fn get_clinic(&self, id: String) -> Result<ClinicModel>;
    async fn list_clinic(&self, ) -> Result<Vec<ClinicModel>>;
    async fn update_clinic(&self, id: String, dto: UpdateClinicDto) -> Result<ClinicModel>;
    async fn delete_clinic(&self, id: String) -> Result<()>;

    async fn create_patient(&self, dto: CreatePatientDto) -> Result<PatientModel>;
    async fn get_patient(&self, id: String) -> Result<PatientModel>;
    async fn list_patient(&self, ) -> Result<Vec<PatientModel>>;
    async fn update_patient(&self, id: String, dto: UpdatePatientDto) -> Result<PatientModel>;
    async fn delete_patient(&self, id: String) -> Result<()>;

    async fn create_doctor(&self, dto: CreateDoctorDto) -> Result<DoctorModel>;
    async fn get_doctor(&self, id: String) -> Result<DoctorModel>;
    async fn list_doctor(&self, ) -> Result<Vec<DoctorModel>>;
    async fn update_doctor(&self, id: String, dto: UpdateDoctorDto) -> Result<DoctorModel>;
    async fn delete_doctor(&self, id: String) -> Result<()>;
}
#[async_trait::async_trait]
pub trait DbDiseasesRepositories{
    async fn create_diseases(&self, dto: CreateDieaseasDto) -> Result<DieseasModel>;
    async fn get_diseases(&self, id: String) -> Result<DieseasModel>;
    async fn list_diseases(&self, ) -> Result<Vec<DieseasModel>>;
    async fn update_diseases(&self, id: String, dto: UpdateDieaseasDto) -> Result<DieseasModel>;
    async fn delete_diseases(&self, id: String) -> Result<()>;

    async fn create_specialization(&self, dto: CreateSpecializationDto) -> Result<SpecializationModel>;
    async fn get_specialization(&self, id: String) -> Result<SpecializationModel>;
    async fn list_specialization(&self, ) -> Result<Vec<SpecializationModel>>;
    async fn update_specialization(&self, id: String, dto: UpdateSpecializationDto) -> Result<SpecializationModel>;
    async fn delete_specialization(&self, id: String) -> Result<()>;
}
#[async_trait::async_trait]
pub trait DbErcRepositories{
    async fn create_erc(&self, dto: CreateErcDto) -> Result<ErcModel>;
    async fn get_erc(&self, id: String) -> Result<ErcModel>;
    async fn list_erc(&self, ) -> Result<Vec<ErcModel>>;
    async fn update_erc(&self, id: String, dto: UpdateErcDto) -> Result<ErcModel>;
    async fn delete_erc(&self, id: String) -> Result<()>;
}
#[async_trait::async_trait]
pub trait DbUsersRepositories{
    async fn create_user(&self, dto: CreateUserDto) -> Result<UserModel>;
    async fn get_user(&self, id: String) -> Result<UserModel>;
    async fn list_user(&self, ) -> Result<Vec<UserModel>>;
    async fn update_user(&self, id: String, dto: UpdateUserDto) -> Result<UserModel>;
    async fn delete_user(&self, id: String) -> Result<()>;
}