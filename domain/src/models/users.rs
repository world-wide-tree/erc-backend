use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserModel{
    pub id: String,
    pub name: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         // Date of Barth
    pub gender: Gender,
    pub role: String,               // RoleName, not id
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female    
}