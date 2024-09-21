use std::sync::Arc;

use crate::repositories::database::DatabaseRepo;

use self::{clinic::{ClinicService, DoctorService, PatientService}, diseases::DieseasService, erc::ErcService, user::UserService};

pub mod user;
pub mod clinic;
pub mod diseases;
pub mod erc;

pub struct AppServices{
    user: Arc<UserService>,
    clinic: Arc<ClinicService>,
    doctor: Arc<DoctorService>,
    patient: Arc<PatientService>,
    diseases: Arc<DieseasService>,
    erc: Arc<ErcService>,
}

impl AppServices{
    pub fn init(db_repo: DatabaseRepo) -> Self{
        Self { 
            user: Arc::new(UserService::new(db_repo.clone())), 
            clinic: Arc::new(ClinicService::new(db_repo.clone())), 
            doctor: Arc::new(DoctorService::new(db_repo.clone())), 
            patient: Arc::new(PatientService::new(db_repo.clone())), 
            diseases: Arc::new(DieseasService::new(db_repo.clone())), 
            erc: Arc::new(ErcService::new(db_repo.clone())) 
        }
    }
    pub fn user_services(&self) -> Arc<UserService>{
        self.user.clone()
    }
    pub fn clinic_services(&self) -> Arc<ClinicService>{
        self.clinic.clone()
    }
    pub fn doctor_services(&self) -> Arc<DoctorService>{
        self.doctor.clone()
    }
    pub fn patient_services(&self) -> Arc<PatientService>{
        self.patient.clone()
    }
    pub fn diseases_services(&self) -> Arc<DieseasService>{
        self.diseases.clone()
    }
    pub fn erc_services(&self) -> Arc<ErcService>{
        self.erc.clone()
    }
}