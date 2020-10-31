#[macro_use]
extern crate diesel;

// mod insert;
mod query;
pub mod models;
pub mod schema;
pub mod repository;

fn main() {
    query::run();
}