use crate::models::User;
use crate::db_provider::connect;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub fn run() {
    let conn = connect().get()
        .map_err(|err| { println!( "get connection from pool error in line:{} ! error: {:?}", line!(), err) })
        .expect("Retrieve the connection...");

    let results: Vec<User> = users
        // .filter(active.eq(true))
        .limit(5)
        .load::<User>(&conn)
        .unwrap();

    
    println!("Displaying {} posts", results.len());
    for item in results {
        println!("User --> {:?}", item);
    }
}