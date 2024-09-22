use chrono::{DateTime, Utc};
use domain::models::erc::{CreateErcDto, ErcModel, UpdateErcDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::clinics::PatientApiDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErcApiDto{
    pub id: String,
    pub patient: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostErcApiDto{
    pub patient: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateErcApiDto{
    pub patient: String,
}

impl Into<ErcModel> for ErcApiDto{
    fn into(self) -> ErcModel {
        ErcModel { 
            id: self.id, 
            patient: self.patient, 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}
impl Into<CreateErcDto> for PostErcApiDto{
    fn into(self) -> CreateErcDto {
        CreateErcDto { 
            patient: self.patient 
        }
    }
}
impl Into<UpdateErcDto> for UpdateErcApiDto{
    fn into(self) -> UpdateErcDto {
        UpdateErcDto { 
            patient: self.patient 
        }
    }
}