use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersDb{
    id: Thing,
    name: String,
    password_hash: String,
    first_name: String,
    last_name: String,
    dob: DateTime<Utc>,
    gender: GenderDb,
    role: Thing,
    phone_number: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleDb{
    id: Thing,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenderDb {
    Male,
    Female    
}