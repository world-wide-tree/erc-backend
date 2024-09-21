use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MedicalHistoriesApiDto{
    pub id: String,
    pub erc: String,                        // TODO: Replace to ErcModel
    pub dieseases: DieseasApiDto,
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