
use crate::models::User;
use crate::repository::establish_connection;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub fn run() {
    let conn = establish_connection();
    let results: Vec<User> = users
        .filter(active.eq(true))
        .limit(5)
        .load::<User>(&conn)
        .unwrap();

    
    println!("Displaying {} posts", results.len());
    for item in results {
        println!("User --> {:?}", item);
    }
}