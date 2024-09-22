use chrono::{DateTime, Utc};
use domain::models::users::{CreateUserDto, Gender, UpdateUserDto, UserModel};
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

impl From<Gender> for GenderApiDto{
    fn from(value: Gender) -> Self {
        match value {
            Gender::Female => Self::Female,
            Gender::Male => Self::Male,
        }
    }
}
impl Into<Gender> for GenderApiDto{
    fn into(self) -> Gender {
        match self{
            Self::Male => Gender::Male,
            Self::Female => Gender::Female
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostUserApiDto{
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
pub struct PutUserApiDto{
    pub name: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,         
    pub gender: GenderApiDto,
    pub role: String,               
    pub phone_number: String,
}

impl Into<UserModel> for UserApiDto{
    fn into(self) -> UserModel {
        UserModel { 
            id: self.id, 
            name: self.name, 
            password_hash: self.password_hash, 
            first_name: self.first_name, 
            last_name: self.last_name, 
            dob: self.dob, 
            gender: self.gender.into(), 
            role: self.role, 
            phone_number: self.phone_number, 
            created_at: self.created_at, 
            updated_at: self.updated_at 
        }
    }
}
impl Into<CreateUserDto> for PostUserApiDto{
    fn into(self) -> CreateUserDto {
        CreateUserDto { 
            name: self.name, 
            password: self.password, 
            first_name: self.first_name, 
            last_name: self.last_name, 
            dob: self.dob, 
            gender: self.gender.into(), 
            role: self.role, 
            phone_number: self.phone_number 
        }
    }
}
impl Into<UpdateUserDto> for PutUserApiDto{
    fn into(self) -> UpdateUserDto {
        UpdateUserDto { 
            name: self.name, 
            first_name: self.first_name, 
            last_name: self.last_name, 
            dob: self.dob, 
            gender: self.gender.into(), 
            role: self.role, 
            phone_number: self.phone_number 
        }
    }
}