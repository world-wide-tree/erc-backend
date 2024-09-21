use chrono::{DateTime, Utc};
use domain::models::erc::{CreateErcDto, ErcModel, UpdateErcDto};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{ERC_TABLE, PATIENT_TABLE};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErcDb{
    id: Thing,
    patient: Thing,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateErcDb{
    pub patient: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateErcDb{
    pub patient: Thing,
}

impl Into<ErcModel> for ErcDb{
    fn into(self) -> ErcModel {
        ErcModel { 
            id: self.id.id.to_string(), 
            patient: self.patient.id.to_string(), 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}

impl From<ErcModel> for ErcDb{
    fn from(value: ErcModel) -> Self {
        Self { 
            id: Thing::from((ERC_TABLE, value.id.as_str())), 
            patient: Thing::from((PATIENT_TABLE, value.patient.as_str())), 
            created_at: value.created_at, 
            updated_at: value.updated_at 
        }
    }
} 
impl From<UpdateErcDto> for UpdateErcDb{
    fn from(value: UpdateErcDto) -> Self {
        Self { 
            patient: Thing::from((PATIENT_TABLE, value.patient.as_str())) 
        }
    }
} 
impl From<CreateErcDto> for CreateErcDb{
    fn from(value: CreateErcDto) -> Self {
        Self { 
            patient: Thing::from((PATIENT_TABLE, value.patient.as_str())) 
        }
    }
} 