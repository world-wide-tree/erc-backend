use chrono::{DateTime, Utc};
use domain::models::diseases::{CreateDieaseasDto, CreateSpecializationDto, DieseasModel, MedicalHistoriesModel, SpecializationModel, UpdateDieaseasDto, UpdateSpecializationDto};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{DISEASES_TABLE, ERC_TABLE, MEDICAL_HISTORIES_TABLE, SPECIALIZATION_TABLE};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieseasesDb{
    id: Thing,
    name: String,
    descriptions: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDieaseasDb{
    pub name: String,
    pub descriptions: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDieaseasDb{
    pub name: String,
    pub descriptions: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializationsDb{
    id: Thing,
    name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpecializationDb{
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpecializationDb{
    pub name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalHistoriesDb{
    id: Thing,
    erc: Thing,
    dieseases: Thing,
    description: String,
    start_data: DateTime<Utc>,
    end_data: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<DieseasModel> for DieseasesDb{
    fn from(value: DieseasModel) -> Self{
        Self { 
            id: Thing::from((DISEASES_TABLE, value.id.as_str())), 
            name: value.name, 
            descriptions: value.descriptions, 
            created_at: value.created_at, 
            updated_at: value.updated_at 
        }
    }
}
impl From<CreateDieaseasDto> for CreateDieaseasDb{
    fn from(value: CreateDieaseasDto) -> Self{
        Self { 
            name: value.name, 
            descriptions: value.descriptions 
        }
    }
}
impl From<UpdateDieaseasDto> for UpdateDieaseasDb{
    fn from(value: UpdateDieaseasDto) -> Self{
        Self { 
            name: value.name, 
            descriptions: value.descriptions 
        }
    }
}
impl From<SpecializationModel> for SpecializationsDb{
    fn from(value: SpecializationModel) -> Self{
        Self { 
            id: Thing::from((SPECIALIZATION_TABLE, value.id.as_str())),
            name: value.name 
        }
    }
}
impl From<CreateSpecializationDto> for CreateSpecializationDb{
    fn from(value: CreateSpecializationDto) -> Self{
        Self { 
            name: value.name 
        }
    }
}
impl From<UpdateSpecializationDto> for UpdateSpecializationDb{
    fn from(value: UpdateSpecializationDto) -> Self{
        Self { 
            name: value.name 
        }
    }   
}
impl From<MedicalHistoriesModel> for MedicalHistoriesDb{
    fn from(value: MedicalHistoriesModel) -> Self{
        Self { 
            id: Thing::from((MEDICAL_HISTORIES_TABLE, value.id.as_str())), 
            erc: Thing::from((ERC_TABLE, value.erc.as_str())), 
            dieseases: Thing::from((DISEASES_TABLE, value.dieseases.as_str())), 
            description: value.description, 
            start_data: value.start_data, 
            end_data: value.end_data, 
            created_at: value.created_at, 
            updated_at: value.updated_at 
        }
    }
}
impl Into<DieseasModel> for DieseasesDb{
    fn into(self) -> DieseasModel{
        DieseasModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            descriptions: self.descriptions, 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}
impl Into<CreateDieaseasDto> for CreateDieaseasDb{
    fn into(self) -> CreateDieaseasDto{
        CreateDieaseasDto { 
            name: self.name, 
            descriptions: self.descriptions 
        }
    }
}
impl Into<UpdateDieaseasDto> for UpdateDieaseasDb{
    fn into(self) -> UpdateDieaseasDto{
        UpdateDieaseasDto { 
            name: self.name, 
            descriptions: self.descriptions 
        }
    }
}
impl Into<SpecializationModel> for SpecializationsDb{
    fn into(self) -> SpecializationModel{
        SpecializationModel { 
            id: self.id.id.to_string(), 
            name: self.name 
        }
    }
}
impl Into<CreateSpecializationDto> for CreateSpecializationDb{
    fn into(self) -> CreateSpecializationDto{
        CreateSpecializationDto { 
            name: self.name 
        }
    }
}
impl Into<UpdateSpecializationDto> for UpdateSpecializationDb{
    fn into(self) -> UpdateSpecializationDto{
        UpdateSpecializationDto { 
            name: self.name 
        }
    }
}
impl Into<MedicalHistoriesModel> for MedicalHistoriesDb{
    fn into(self) -> MedicalHistoriesModel{
        MedicalHistoriesModel { 
            id: self.id.id.to_string(), 
            erc: self.erc.id.to_string(), 
            dieseases: self.dieseases.id.to_string(), 
            description: self.description, 
            start_data: self.start_data, 
            end_data: self.end_data, 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}