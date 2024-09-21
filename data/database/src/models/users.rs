use chrono::{DateTime, Utc};
use domain::models::users::{CreateUserDto, Gender, UpdateUserDto, UserModel};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::ROLES_TABLE;

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
impl From<Gender> for GenderDb{
    fn from(value: Gender) -> Self {
        match value {
            Gender::Female => Self::Female,
            Gender::Male => Self::Male,
        }
    }
}
impl Into<Gender> for GenderDb{
    fn into(self) -> Gender {
        match self{
            Self::Male => Gender::Male,
            Self::Female => Gender::Female
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDb{
    pub name: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         
    pub gender: GenderDb,
    pub role: Thing,               
    pub phone_number: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDb{
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         
    pub gender: GenderDb,
    pub role: Thing,               
    pub phone_number: String,
}

impl Into<UserModel> for UsersDb { 
    fn into(self) -> UserModel {
        UserModel { 
            id: self.id.id.to_string(), 
            name: self.name, 
            password_hash: self.password_hash, 
            first_name: self.first_name, 
            last_name: self.last_name, 
            dob: self.dob, 
            gender: self.gender.into(), 
            role: self.role.id.to_string(), 
            phone_number: self.phone_number, 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}

impl From<CreateUserDto> for CreateUserDb { 
    fn from(value: CreateUserDto) -> Self {
        Self { 
            name: value.name, 
            password_hash: value.password, 
            first_name: value.first_name, 
            last_name: value.last_name, 
            dob: value.dob, 
            gender: value.gender.into(), 
            role: Thing::from((ROLES_TABLE, value.role.as_str())), 
            phone_number: value.phone_number }
    }
}
impl From<UpdateUserDto> for UpdateUserDb { 
    fn from(value: UpdateUserDto) -> Self {
        Self { 
            name: value.name, 
            first_name: value.first_name, 
            last_name: value.last_name, 
            dob: value.dob, 
            gender: value.gender.into(), 
            role: Thing::from((ROLES_TABLE, value.role.as_str())), 
            phone_number: value.phone_number 
        }
    }
}