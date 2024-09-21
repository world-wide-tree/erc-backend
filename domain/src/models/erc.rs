use chrono::{DateTime, Utc};

use super::clinics::PatientModel;

#[derive(Debug, Clone)]
pub struct ErcModel{
    pub id: String,
    pub patient: PatientModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone)]
pub struct CreateErcDto{
    pub patient: String,
}
#[derive(Debug, Clone)]
pub struct UpdateErcDto{
    pub patient: String,
}