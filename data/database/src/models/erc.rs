use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErcDb{
    id: Thing,
    patient: Thing,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}