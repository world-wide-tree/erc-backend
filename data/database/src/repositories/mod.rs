use anyhow::Result;
use domain::{models::{clinics::{ClinicModel, CreateClinicDto, CreateDoctorDto, CreatePatientDto, DoctorModel, PatientModel, UpdateClinicDto, UpdateDoctorDto, UpdatePatientDto}, diseases::{CreateDieaseasDto, CreateSpecializationDto, DieseasModel, SpecializationModel, UpdateDieaseasDto, UpdateSpecializationDto}, erc::{CreateErcDto, ErcModel, UpdateErcDto}, users::{CreateUserDto, UpdateUserDto, UserModel}}, repositories::database::{DatabaseRepositories, DbClinicsRepositories, DbDiseasesRepositories, DbErcRepositories, DbUsersRepositories}};

use crate::pool::DbClient;

#[async_trait::async_trait]
impl DatabaseRepositories for DbClient{}

#[async_trait::async_trait]
impl DbClinicsRepositories for DbClient{
    async fn create_clinic(&self, dto: CreateClinicDto) -> Result<ClinicModel>{
        todo!("create_clinic")
    }
    async fn get_clinic(&self, id: String) -> Result<ClinicModel>{
        todo!("get_clinic")
    }
    async fn list_clinic(&self, ) -> Result<Vec<ClinicModel>>{
        todo!("list_clinic")
    }
    async fn update_clinic(&self, id: String, dto: UpdateClinicDto) -> Result<ClinicModel>{
        todo!("update_clinic")
    }
    async fn delete_clinic(&self, id: String) -> Result<()>{
        todo!("delete_clinic")
    }

    async fn create_patient(&self, dto: CreatePatientDto) -> Result<PatientModel>{
        todo!("create_patient")
    }
    async fn get_patient(&self, id: String) -> Result<PatientModel>{
        todo!("get_patient")
    }
    async fn list_patient(&self, ) -> Result<Vec<PatientModel>>{
        todo!("list_patient")
    }
    async fn update_patient(&self, id: String, dto: UpdatePatientDto) -> Result<ClinicModel>{
        todo!("update_patient")
    }
    async fn delete_patient(&self, id: String) -> Result<()>{
        todo!("delete_patient")
    }

    async fn create_doctor(&self, dto: CreateDoctorDto) -> Result<DoctorModel>{
        todo!("create_doctor")
    }
    async fn get_doctor(&self, id: String) -> Result<DoctorModel>{
        todo!("get_doctor")
    }
    async fn list_doctor(&self, ) -> Result<Vec<DoctorModel>>{
        todo!("list_doctor")
    }
    async fn update_doctor(&self, id: String, dto: UpdateDoctorDto) -> Result<DoctorModel>{
        todo!("update_doctor")
    }
    async fn delete_doctor(&self, id: String) -> Result<()>{
        todo!("delete_doctor")
    }
}
#[async_trait::async_trait]
impl DbDiseasesRepositories for DbClient{
    async fn create_diseases(&self, dto: CreateDieaseasDto) -> Result<DieseasModel>{
        todo!("create_diseases")
    }
    async fn get_diseases(&self, id: String) -> Result<DieseasModel>{
        todo!("get_diseases")
    }
    async fn list_diseases(&self, ) -> Result<Vec<DieseasModel>>{
        todo!("list_diseases")
    }
    async fn update_diseases(&self, id: String, dto: UpdateDieaseasDto) -> Result<DieseasModel>{
        todo!("update_diseases")
    }
    async fn delete_diseases(&self, id: String) -> Result<()>{
        todo!("delete_diseases")
    }

    async fn create_specialization(&self, dto: CreateSpecializationDto) -> Result<SpecializationModel>{
        todo!("create_specialization")
    }
    async fn get_specialization(&self, id: String) -> Result<SpecializationModel>{
        todo!("get_specialization")
    }
    async fn list_specialization(&self, ) -> Result<Vec<SpecializationModel>>{
        todo!("list_specialization")
    }
    async fn update_specialization(&self, id: String, dto: UpdateSpecializationDto) -> Result<SpecializationModel>{
        todo!("update_specialization")
    }
    async fn delete_specialization(&self, id: String) -> Result<()>{
        todo!("delete_specialization")
    }
}
#[async_trait::async_trait]
impl DbErcRepositories for DbClient{
    async fn create_erc(&self, dto: CreateErcDto) -> Result<ErcModel>{
        todo!("create_erc")
    }
    async fn get_erc(&self, id: String) -> Result<ErcModel>{
        todo!("get_erc")
    }
    async fn list_erc(&self, ) -> Result<Vec<ErcModel>>{
        todo!("list_erc")
    }
    async fn update_erc(&self, id: String, dto: UpdateErcDto) -> Result<ErcModel>{
        todo!("update_erc")
    }
    async fn delete_erc(&self, id: String) -> Result<ErcModel>{
        todo!("delete_erc")
    }
}
#[async_trait::async_trait]
impl DbUsersRepositories for DbClient{
    async fn create_user(&self, dto: CreateUserDto) -> Result<UserModel>{
        todo!("create_user")
    }
    async fn get_user(&self, id: String) -> Result<UserModel>{
        todo!("get_user")
    }
    async fn list_user(&self, ) -> Result<Vec<UserModel>>{
        todo!("list_user")
    }
    async fn update_user(&self, id: String, dto: UpdateUserDto) -> Result<UserModel>{
        todo!("update_user")
    }
    async fn delete_user(&self, id: String) -> Result<()>{
        todo!("delete_user")
    }
}
