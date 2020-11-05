#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod adapter;
mod application;
mod domain;
mod infrastructure;

fn main() {
    infrastructure::db_provider::connect_to_datbase();
    infrastructure::http_server::launch();
}