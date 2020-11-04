use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::models::{UserData, NewUserData, UserChangeBodyDto};
use crate::db_provider::stablish_connection;

use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::result::Error;

pub fn create_user(user: UserChangeBodyDto) -> UserData {
    let conn = stablish_connection();
    let user_to_persist = NewUserData {
        id: &0,
        first_name: &user.first_name,
        last_name: &user.last_name,
        phone: &user.phone,
    };

    conn.transaction::<_, Error, _>(|| {
        diesel::insert_into(users::table)
            .values(&user_to_persist)
            .execute(&conn)
            .unwrap();

        users.order(id.desc())
            .first::<UserData>(&conn)
    }).unwrap()
}

pub fn delete_user_by_id(user_id: i32) {
    let conn = stablish_connection();
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&conn)
        .unwrap();

}

pub fn find_all_users() -> Vec<UserData> {
    let conn = stablish_connection();
    users
        .load::<UserData>(&conn)
        .unwrap()
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