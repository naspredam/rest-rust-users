extern crate chrono;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod repository;

use models::{NewUser};
use crate::diesel::RunQueryDsl;

fn main() {
    let new_user = NewUser {
        id: &1,
        first_name: "Manel",
        last_name: "Naspreda",
        phone: "+34...",
        active: &true,
    };

    let conn = repository::establish_connection();
    let result = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&conn);

    println!("{:?}", result);
}