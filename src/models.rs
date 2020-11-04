use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Serialize)]
pub struct UserData {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUserData<'a> {
    pub id: &'a i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub phone: &'a str,
}

#[derive(Deserialize)] 
pub struct UserChangeBodyDto {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}