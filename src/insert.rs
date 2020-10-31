
use crate::schema::users;
use crate::models::{NewUser};
use crate::db_provider::establish_connection;
use diesel::RunQueryDsl;

pub fn run() {
    let new_user = NewUser {
        id: &1,
        first_name: "Manel",
        last_name: "Naspreda",
        phone: "+34...",
        active: &true,
    };

    let conn = establish_connection();
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn);

    println!("{:?}", result);

}