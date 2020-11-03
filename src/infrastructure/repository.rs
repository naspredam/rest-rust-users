use crate::domain::models::User;
use crate::infrastructure::db_provider::stablish_connection;
use crate::infrastructure::mapper;
use crate::infrastructure::schema::users::dsl::*;
use crate::infrastructure::schema::users;
use crate::infrastructure::models::{NewUserData, UserData};

use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::result::Error;

use std::option::Option;

pub fn create_user(user: User) -> User {
    let conn = stablish_connection();
    let num_users: i64 = users.count().get_result(&conn).unwrap();
    let user_to_persist = NewUserData {
        id: &(num_users as i32 + 1),
        first_name: &user.first_name,
        last_name: &user.last_name,
        phone: &user.phone,
        active: &user.active,
    };

    conn.transaction::<_, Error, _>(|| {
        diesel::insert_into(users::table)
            .values(&user_to_persist)
            .execute(&conn)
            .unwrap();

        users.order(id.desc())
            .first::<UserData>(&conn)
            .map(mapper::map_to_domain)
    }).unwrap()
}

pub fn find_user_by_id(user_id: i32) -> Option<User> {
    let conn = stablish_connection();
    let user_found_result = users
        .filter(id.eq(user_id))
        .first::<UserData>(&conn)
        .map(mapper::map_to_domain);

    match user_found_result {
        Ok(user_found) => Some(user_found),
        Err(..) => None
    }
}

pub fn find_all_users() -> Vec<User> {
    let conn = stablish_connection();
    users
        .load::<UserData>(&conn)
        .unwrap()
        .into_iter()
        .map(mapper::map_to_domain)
        .rev()
        .collect()
}

pub fn delete_user_by_id(user_id: i32) {
    let conn = stablish_connection();
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&conn)
        .unwrap();

}