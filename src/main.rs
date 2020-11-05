#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod adapter;
mod application;
mod domain;

use adapter::inbound::http::http_server;
use adapter::outbound::persistence::db_provider;

fn main() {
    db_provider::connect_to_datbase();
    http_server::launch();
}