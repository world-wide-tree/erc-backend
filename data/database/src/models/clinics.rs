use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorsDb{
    id: Thing,
    user: Thing,
    specialization: Thing,
    clinic: Thing
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientsDb{
    id: Thing,
    user: Thing,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicsDb{
    id: Thing,
    name: String,
    address: String,
    contact_info: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}