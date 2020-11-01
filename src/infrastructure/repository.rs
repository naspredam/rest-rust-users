use crate::infrastructure::db_provider::connect;
use crate::infrastructure::schema::users::dsl::*;
use crate::infrastructure::schema::users;
use crate::infrastructure::models::{NewUser, NewUser1, User};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::result::Error;

pub fn create_user() -> User {
    let new_user = NewUser1 {
        first_name: "Manel",
        last_name: "Naspreda",
        phone: "+34...",
        active: &true,
    };

    let conn = connect().get()
        .map_err(|err| { println!( "get connection from pool error in line:{} ! error: {:?}", line!(), err) })
        .expect("Retrieve the connection...");
    let num_users: i64 = users.count().get_result(&conn).expect("");
    let next_user_id: i32 = num_users as i32 + 1;
    let user_to_persist = NewUser {
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

            users.order(id.desc()).first::<User>(&conn)
        }).expect("User saved not able to retrieve data...")
}

// pub fn findUserById() -> Optional<User> {
    
// }

// pub fn findAllUsers() -> Vec<User> {
    
// }

// pub fn deleteUserById() {
    
// }