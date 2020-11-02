#[macro_use]
extern crate diesel;

mod infrastructure;
mod starters;

use infrastructure::repository;

fn main() {
    starters::start();
    let user = repository::find_all_users();
    println!("{:?}", user);
}