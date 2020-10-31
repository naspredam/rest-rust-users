use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub id: &'a i32,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub phone: &'a str,
    pub active: &'a bool,
}