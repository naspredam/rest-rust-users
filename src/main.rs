#[macro_use]
extern crate diesel;

mod infrastructure;
mod domain;
mod starters;

use infrastructure::repository;

fn main() {
    starters::start();
    let user = repository::find_all_users();
    println!("{:?}", user);
}