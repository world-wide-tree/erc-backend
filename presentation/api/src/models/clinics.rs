use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DoctorApiDto{
    pub id: String,
    pub user: String,
    pub specialization: String,
    pub clinic: String
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PatientApiDto{
    pub id: String,
    pub user: String        
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