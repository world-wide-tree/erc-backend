use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieseasesDb{
    id: Thing,
    name: String,
    descriptions: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializationsDb{
    id: Thing,
    name: String,
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