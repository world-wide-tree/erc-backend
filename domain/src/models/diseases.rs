use chrono::{DateTime, Utc};

use super::erc::ErcModel;

#[derive(Debug, Clone)]
pub struct DieseasModel{
    pub id: String,
    pub name: String,
    pub descriptions: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone)]
pub struct SpecializationModel{
    pub id: String,
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct MedicalHistoriesDb{
    pub id: String,
    pub erc: ErcModel,                        // TODO: Replace to ErcModel
    pub dieseases: DieseasModel,
    pub description: String,
    pub start_data: DateTime<Utc>,
    pub end_data: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}