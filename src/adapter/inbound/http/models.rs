use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)] 
pub struct UserDto {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub active: bool,
}

#[derive(Deserialize)] 
pub struct UserChangeBodyDto {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub active: bool,
}