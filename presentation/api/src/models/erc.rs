use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::clinics::PatientApiDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErcApiDto{
    pub id: String,
    pub patient: PatientApiDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}