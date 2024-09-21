use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

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
    updated_at: DateTime<Utc>,
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