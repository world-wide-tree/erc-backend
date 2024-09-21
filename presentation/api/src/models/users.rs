use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserApiDto{
    pub id: String,
    pub name: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         // Date of Barth
    pub gender: GenderApiDto,
    pub role: String,               // RoleName, not id
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum GenderApiDto {
    Male,
    Female    
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUserApiDto{
    pub name: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         
    pub gender: GenderApiDto,
    pub role: String,               
    pub phone_number: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserApiDto{
    pub name: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         
    pub gender: GenderApiDto,
    pub role: String,               
    pub phone_number: String,
}