use crate::adapter::outbound::persistence::schema::users;

#[derive(Debug, Queryable)]
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
