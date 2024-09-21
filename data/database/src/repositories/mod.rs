use anyhow::{anyhow, bail, Result};
use domain::{models::{clinics::{ClinicModel, CreateClinicDto, CreateDoctorDto, CreatePatientDto, DoctorModel, PatientModel, UpdateClinicDto, UpdateDoctorDto, UpdatePatientDto}, diseases::{CreateDieaseasDto, CreateSpecializationDto, DieseasModel, SpecializationModel, UpdateDieaseasDto, UpdateSpecializationDto}, erc::{CreateErcDto, ErcModel, UpdateErcDto}, users::{CreateUserDto, UpdateUserDto, UserModel}}, repositories::database::{DatabaseRepositories, DbClinicsRepositories, DbDiseasesRepositories, DbErcRepositories, DbUsersRepositories}};

use crate::{models::{clinics::{ClinicsDb, CreateClinicDb, CreateDoctorDb, CreatePatientDb, DoctorsDb, PatientsDb, UpdateClinicDb, UpdateDoctorDb, UpdatePatientDb}, diseases::{CreateDieaseasDb, CreateSpecializationDb, DieseasesDb, SpecializationsDb, UpdateDieaseasDb, UpdateSpecializationDb}, erc::{CreateErcDb, ErcDb, UpdateErcDb}, users::{CreateUserDb, UpdateUserDb, UsersDb}, CLINIC_TABLE, DISEASES_TABLE, DOCTOR_TABLE, ERC_TABLE, PATIENT_TABLE, SPECIALIZATION_TABLE, USER_TABLE}, pool::DbClient};

#[async_trait::async_trait]
impl DatabaseRepositories for DbClient{}

#[async_trait::async_trait]
impl DbClinicsRepositories for DbClient{
    async fn create_clinic(&self, dto: CreateClinicDto) -> Result<ClinicModel>{
        let rst: Option<ClinicsDb> = self.get()
            .create(CLINIC_TABLE)
            .content(CreateClinicDb::from(dto))
            .await?
        ;
        rst.map(ClinicsDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_clinic(&self, id: String) -> Result<ClinicModel>{
        let rst: Option<ClinicsDb> = self.get()
            .select((CLINIC_TABLE, id.as_str()))
            .await?
        ;
        rst.map(ClinicsDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_clinic(&self) -> Result<Vec<ClinicModel>>{
        let rst: Vec<ClinicsDb> = self.get()
            .select(CLINIC_TABLE)
            .await?;
        Ok(rst.into_iter().map(ClinicsDb::into).collect())
    }
    async fn update_clinic(&self, id: String, dto: UpdateClinicDto) -> Result<ClinicModel>{
        let rst: Option<ClinicsDb> = self.get()
            .update((CLINIC_TABLE, id.as_str()))
            .merge(UpdateClinicDb::from(dto))
            .await?
        ;
        rst.map(ClinicsDb::into).ok_or(anyhow!("Cannot update clinic! Not Found on db!"))
    }
    async fn delete_clinic(&self, id: String) -> Result<()>{
        let rst: Option<ClinicsDb> = self.get()
            .delete((CLINIC_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
    }

    async fn create_patient(&self, dto: CreatePatientDto) -> Result<PatientModel>{
        let rst: Option<PatientsDb> = self.get()
            .create(PATIENT_TABLE)
            .content(CreatePatientDb::from(dto))
            .await?
        ;
        rst.map(PatientsDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_patient(&self, id: String) -> Result<PatientModel>{
        let rst: Option<PatientsDb> = self.get()
            .select((PATIENT_TABLE, id.as_str()))
            .await?
        ;
        rst.map(PatientsDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_patient(&self, ) -> Result<Vec<PatientModel>>{
        let rst: Vec<PatientsDb> = self.get()
            .select(PATIENT_TABLE)
            .await?;
        Ok(rst.into_iter().map(PatientsDb::into).collect())
    }
    async fn update_patient(&self, id: String, dto: UpdatePatientDto) -> Result<PatientModel>{
        let rst: Option<PatientsDb> = self.get()
            .update((PATIENT_TABLE, id.as_str()))
            .merge(UpdatePatientDb::from(dto))
            .await?
        ;
        rst.map(PatientsDb::into).ok_or(anyhow!("Cannot update clinic! Not Found on db!"))
    }
    async fn delete_patient(&self, id: String) -> Result<()>{
        let rst: Option<PatientsDb> = self.get()
            .delete((PATIENT_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
    }

    async fn create_doctor(&self, dto: CreateDoctorDto) -> Result<DoctorModel>{
        let rst: Option<DoctorsDb> = self.get()
            .create(DOCTOR_TABLE)
            .content(CreateDoctorDb::from(dto))
            .await?
        ;
        rst.map(DoctorsDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_doctor(&self, id: String) -> Result<DoctorModel>{
        let rst: Option<DoctorsDb> = self.get()
            .select((DOCTOR_TABLE, id.as_str()))
            .await?
        ;
        rst.map(DoctorsDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_doctor(&self, ) -> Result<Vec<DoctorModel>>{
        let rst: Vec<DoctorsDb> = self.get()
            .select(DOCTOR_TABLE)
            .await?;
        Ok(rst.into_iter().map(DoctorsDb::into).collect())
    }
    async fn update_doctor(&self, id: String, dto: UpdateDoctorDto) -> Result<DoctorModel>{
        let rst: Option<DoctorsDb> = self.get()
            .update((DOCTOR_TABLE, id.as_str()))
            .merge(UpdateDoctorDb::from(dto))
            .await?
        ;
        rst.map(DoctorsDb::into).ok_or(anyhow!("Cannot update clinic! Not Found on db!"))
    }
    async fn delete_doctor(&self, id: String) -> Result<()>{
               let rst: Option<DoctorsDb> = self.get()
            .delete((DOCTOR_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
        
    }
}
#[async_trait::async_trait]
impl DbDiseasesRepositories for DbClient{
    async fn create_diseases(&self, dto: CreateDieaseasDto) -> Result<DieseasModel>{
        let rst: Option<DieseasesDb> = self.get()
            .create(DISEASES_TABLE)
            .content(CreateDieaseasDb::from(dto))
            .await?
        ;
        rst.map(DieseasesDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_diseases(&self, id: String) -> Result<DieseasModel>{
        let rst: Option<DieseasesDb> = self.get()
            .select((DISEASES_TABLE, id.as_str()))
            .await?
        ;
        rst.map(DieseasesDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_diseases(&self, ) -> Result<Vec<DieseasModel>>{
        let rst: Vec<DieseasesDb> = self.get()
            .select(DISEASES_TABLE)
            .await?;
        Ok(rst.into_iter().map(DieseasesDb::into).collect())
    }
    async fn update_diseases(&self, id: String, dto: UpdateDieaseasDto) -> Result<DieseasModel>{
        let rst: Option<DieseasesDb> = self.get()
            .update((DISEASES_TABLE, id.as_str()))
            .merge(UpdateDieaseasDb::from(dto))
            .await?
        ; 
        rst.map(DieseasesDb::into).ok_or(anyhow!("Cannot update Diseases! Not Found on db!"))
    }
    async fn delete_diseases(&self, id: String) -> Result<()>{
               let rst: Option<DieseasesDb> = self.get()
            .delete((DISEASES_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
        
    }

    async fn create_specialization(&self, dto: CreateSpecializationDto) -> Result<SpecializationModel>{
        let rst: Option<SpecializationsDb> = self.get()
            .create(SPECIALIZATION_TABLE)
            .content(CreateSpecializationDb::from(dto))
            .await?
        ;
        rst.map(SpecializationsDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_specialization(&self, id: String) -> Result<SpecializationModel>{
        let rst: Option<SpecializationsDb> = self.get()
            .select((SPECIALIZATION_TABLE, id.as_str()))
            .await?
        ;
        rst.map(SpecializationsDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_specialization(&self, ) -> Result<Vec<SpecializationModel>>{
        let rst: Vec<SpecializationsDb> = self.get()
            .select(SPECIALIZATION_TABLE)
            .await?;
        Ok(rst.into_iter().map(SpecializationsDb::into).collect())
    }
    async fn update_specialization(&self, id: String, dto: UpdateSpecializationDto) -> Result<SpecializationModel>{
        let rst: Option<SpecializationsDb> = self.get()
            .update((SPECIALIZATION_TABLE, id.as_str()))
            .merge(UpdateSpecializationDb::from(dto))
            .await?
        ;
        rst.map(SpecializationsDb::into).ok_or(anyhow!("Cannot update clinic! Not Found on db!"))
    }
    async fn delete_specialization(&self, id: String) -> Result<()>{
               let rst: Option<SpecializationsDb> = self.get()
            .delete((SPECIALIZATION_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
        
    }
}
#[async_trait::async_trait]
impl DbErcRepositories for DbClient{
    async fn create_erc(&self, dto: CreateErcDto) -> Result<ErcModel>{
        let rst: Option<ErcDb> = self.get()
            .create(ERC_TABLE)
            .content(CreateErcDb::from(dto))
            .await?
        ;
        rst.map(ErcDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_erc(&self, id: String) -> Result<ErcModel>{
        let rst: Option<ErcDb> = self.get()
            .select((ERC_TABLE, id.as_str()))
            .await?
        ;
        rst.map(ErcDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_erc(&self, ) -> Result<Vec<ErcModel>>{
        let rst: Vec<ErcDb> = self.get()
            .select(ERC_TABLE)
            .await?;
        Ok(rst.into_iter().map(ErcDb::into).collect())
    }
    async fn update_erc(&self, id: String, dto: UpdateErcDto) -> Result<ErcModel>{
        let rst: Option<ErcDb> = self.get()
            .update((ERC_TABLE, id.as_str()))
            .merge(UpdateErcDb::from(dto))
            .await?
        ;
        rst.map(ErcDb::into).ok_or(anyhow!("Cannot update clinic! Not Found on db!"))
    }
    async fn delete_erc(&self, id: String) -> Result<()>{
        let rst: Option<ErcDb> = self.get()
            .delete((ERC_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
        
    }
}
#[async_trait::async_trait]
impl DbUsersRepositories for DbClient{
    async fn create_user(&self, dto: CreateUserDto) -> Result<UserModel>{
        let rst: Option<UsersDb> = self.get()
            .create(USER_TABLE)
            .content(CreateUserDb::from(dto))
            .await?
        ;
        rst.map(UsersDb::into).ok_or(anyhow!("Cannot created record on db!"))
    }
    async fn get_user(&self, id: String) -> Result<UserModel>{
        let rst: Option<UsersDb> = self.get()
            .select((USER_TABLE, id.as_str()))
            .await?
        ;
        rst.map(UsersDb::into).ok_or(anyhow!("Cannot finded clininc on db!"))
    }
    async fn list_user(&self, ) -> Result<Vec<UserModel>>{
        let rst: Vec<UsersDb> = self.get()
            .select(USER_TABLE)
            .await?;
        Ok(rst.into_iter().map(UsersDb::into).collect())
    }
    async fn update_user(&self, id: String, dto: UpdateUserDto) -> Result<UserModel>{
        let rst: Option<UsersDb> = self.get()
            .update((USER_TABLE, id.as_str()))
            .merge(UpdateUserDb::from(dto))
            .await?
        ;
        rst.map(UsersDb::into).ok_or(anyhow!("Cannot update clinic! Not Found on db!"))
    }
    async fn delete_user(&self, id: String) -> Result<()>{
        let rst: Option<UsersDb> = self.get()
            .delete((USER_TABLE, id.as_str()))
            .await?
        ;
        rst.map(|_| ()).ok_or(anyhow!("Cannot delete clinic form db! Not found!"))
        
    }
}
