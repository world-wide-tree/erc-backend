use chrono::{DateTime, Utc};
use domain::models::diseases::{CreateDieaseasDto, CreateSpecializationDto, DieseasModel, MedicalHistoriesModel, SpecializationModel, UpdateDieaseasDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MedicalHistoriesApiDto{
    pub id: String,
    pub erc: String,                        // TODO: Replace to ErcModel
    pub dieseases: String,
    pub description: String,
    pub start_data: DateTime<Utc>,
    pub end_data: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DieseasApiDto{
    pub id: String,
    pub name: String,
    pub descriptions: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostDieaseasApiDto{
    pub name: String,
    pub descriptions: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PutDieaseasApiDto{
    pub name: String,
    pub descriptions: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SpecializationApiDto{
    pub id: String,
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostSpecializationApiDto{
    pub name: String,
}

impl Into<MedicalHistoriesModel> for MedicalHistoriesApiDto{
    fn into(self) -> MedicalHistoriesModel {
        MedicalHistoriesModel { 
            id: self.id, 
            erc: self.erc, 
            dieseases: self.dieseases, 
            description: self.description, 
            start_data: self.start_data, 
            end_data: self.end_data, 
            created_at: self.created_at, 
            updated_at: self.updated_at }
    }
}
impl Into<DieseasModel> for DieseasApiDto{
    fn into(self) -> DieseasModel {
        DieseasModel { 
            id: self.id, 
            name: self.name, 
            descriptions: self.descriptions, 
            created_at: self.created_at, 
            updated_at: self.updated_at }
    }
}
impl Into<CreateDieaseasDto> for PostDieaseasApiDto{
    fn into(self) -> CreateDieaseasDto {
        CreateDieaseasDto { 
            name: self.name, 
            descriptions: self.descriptions }
    }
}
impl Into<UpdateDieaseasDto> for PutDieaseasApiDto{
    fn into(self) -> UpdateDieaseasDto {
        UpdateDieaseasDto { 
            name: self.name, 
            descriptions: self.descriptions }
    }
}
impl Into<SpecializationModel> for SpecializationApiDto{
    fn into(self) -> SpecializationModel {
        SpecializationModel { 
            id: self.id, 
            name: self.name }
    }
}
impl Into<CreateSpecializationDto> for PostSpecializationApiDto{
    fn into(self) -> CreateSpecializationDto {
        CreateSpecializationDto { 
            name: self.name
        }
    }
}