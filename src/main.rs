#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
extern crate rocket;

mod db_provider;
mod http_server;
mod models;
mod schema;
mod repository;

fn main() {
    db_provider::start_connection_pool();
    http_server::launch();
}