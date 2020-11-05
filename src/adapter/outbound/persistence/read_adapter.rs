use crate::domain::models::User;
use crate::infrastructure::db_provider::stablish_connection;
use crate::adapter::outbound::persistence::mapper;
use crate::adapter::outbound::persistence::schema::users::dsl::*;
use crate::adapter::outbound::persistence::models::UserData;

use diesel::prelude::*;
use diesel::RunQueryDsl;

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