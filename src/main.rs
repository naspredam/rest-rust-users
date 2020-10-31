#[macro_use]
extern crate diesel;

mod insert;
pub mod models;
pub mod schema;
pub mod repository;

fn main() {
    insert::run();
}