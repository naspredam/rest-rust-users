use std::option::Option;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}