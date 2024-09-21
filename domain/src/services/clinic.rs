use anyhow::Result;

use crate::{models::clinics::{ClinicModel, CreateClinicDto, CreateDoctorDto, CreatePatientDto, DoctorModel, PatientModel, UpdateClinicDto, UpdateDoctorDto, UpdatePatientDto}, repositories::database::DatabaseRepo};

pub struct ClinicService{
    db: DatabaseRepo
}

impl ClinicService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(&self, dto: CreateClinicDto) -> Result<ClinicModel>{
        self.db.create_clinic(dto).await        
    }
    pub async fn get_by_id(&self, id: String) -> Result<ClinicModel>{
        self.db.get_clinic(id).await 
    }
    pub async fn list(&self) -> Result<Vec<ClinicModel>>{
        self.db.list_clinic().await 
    }
    pub async fn update(&self, id: String, dto: UpdateClinicDto ) -> Result<ClinicModel>{
        self.db.update_clinic(id, dto).await 
    }
    pub async fn delete(&self, id: String) -> Result<()>{
        self.db.delete_clinic(id).await 
    }
}


pub struct DoctorService{
    db: DatabaseRepo
}

impl DoctorService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(&self, dto: CreateDoctorDto) -> Result<DoctorModel>{
        self.db.create_doctor(dto).await
    }
    pub async fn get_by_id(&self, id: String) -> Result<DoctorModel>{
        self.db.get_doctor(id).await 
    }
    pub async fn list(&self) -> Result<Vec<DoctorModel>>{
        self.db.list_doctor().await 
    }
    pub async fn update(&self, id: String, dto: UpdateDoctorDto ) -> Result<DoctorModel>{
        self.db.update_doctor(id, dto).await 
    }
    pub async fn delete(&self, id: String) -> Result<()>{
        self.db.delete_doctor(id).await 
    }
}

pub struct PatientService{
    db: DatabaseRepo
}

impl PatientService{
    pub fn new(db: DatabaseRepo) -> Self{
        Self { db }
    }
    pub async fn create(&self, dto: CreatePatientDto) -> Result<PatientModel>{
        self.db.create_patient(dto).await        
    }
    pub async fn get_by_id(&self, id: String) -> Result<PatientModel>{
        self.db.get_patient(id).await 
    }
    pub async fn list(&self) -> Result<Vec<PatientModel>>{
        self.db.list_patient().await 
    }
    pub async fn update(&self, id: String, dto: UpdatePatientDto ) -> Result<PatientModel>{
        self.db.update_patient(id, dto).await 
    }
    pub async fn delete(&self, id: String) -> Result<()>{
        self.db.delete_patient(id).await 
    }
}
