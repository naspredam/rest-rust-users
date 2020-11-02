use crate::infrastructure::db_provider::stablish_connection;
use crate::infrastructure::schema::users::dsl::*;
use crate::infrastructure::schema::users;
use crate::infrastructure::models::{NewUserData, NewUser1, UserData};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::result::Error;
use std::option::Option;

pub fn create_user() -> UserData {
    let new_user = NewUser1 {
        first_name: "Manel",
        last_name: "Naspreda",
        phone: "+34...",
        active: &true,
    };

    let conn = stablish_connection();
    let num_users: i64 = users.count()
        .get_result(&conn)
        .expect("Count of users persisted.");
    let next_user_id: i32 = num_users as i32 + 1;
    let user_to_persist = NewUserData {
        id: &next_user_id,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        phone: new_user.phone,
        active: new_user.active,
    };

    conn.transaction::<_, Error, _>(|| {
        diesel::insert_into(users::table)
            .values(&user_to_persist)
            .execute(&conn)
            .unwrap();

        users.order(id.desc()).first::<UserData>(&conn)
    }).expect("User saved not able to retrieve data...")
}

pub fn find_user_by_id(user_id: i32) -> Option<UserData> {
    let conn = stablish_connection();
    let user_found_result = users
        .filter(id.eq(user_id))
        .first::<UserData>(&conn);

    match user_found_result {
        Ok(user_found) => Some(user_found),
        Err(..) => None
    }
}

pub fn find_all_users() -> Vec<UserData> {
    let conn = stablish_connection();
    users
        .load::<UserData>(&conn)
        .expect("User saved not able to retrieve data...")

}

// pub fn deleteUserById() {
    
// }