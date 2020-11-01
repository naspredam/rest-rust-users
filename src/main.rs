#[macro_use]
extern crate diesel;

mod infrastructure;

use infrastructure::repository;

fn main() {
    let user = repository::create_user();
    println!("{:?}", user);
}