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
pub struct CreateDieaseasDto{
    pub name: String,
    pub descriptions: String,
}
#[derive(Debug, Clone)]
pub struct UpdateDieaseasDto{
    pub name: String,
    pub descriptions: String,
}
#[derive(Debug, Clone)]
pub struct SpecializationModel{
    pub id: String,
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct CreateSpecializationDto{
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct UpdateSpecializationDto{
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct MedicalHistoriesModel{
    pub id: String,
    pub erc: String,                        // TODO: Replace to ErcModel
    pub dieseases: String,
    pub description: String,
    pub start_data: DateTime<Utc>,
    pub end_data: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
