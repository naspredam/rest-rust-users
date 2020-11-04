use crate::domain::models::User;
use crate::adapter::outbound::persistence::db_provider::stablish_connection;
use crate::adapter::outbound::persistence::mapper;
use crate::adapter::outbound::persistence::schema::users::dsl::*;
use crate::adapter::outbound::persistence::schema::users;
use crate::adapter::outbound::persistence::models::{NewUserData, UserData};

use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::result::Error;

pub fn create_user(user: User) -> User {
    let conn = stablish_connection();
    let user_to_persist = NewUserData {
        id: &0,
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

pub fn delete_user_by_id(user_id: i32) {
    let conn = stablish_connection();
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&conn)
        .unwrap();

}