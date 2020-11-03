#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
extern crate rocket;

mod application;
mod domain;
mod infrastructure;

use application as app;
use infrastructure::starters;

fn main() {
    starters::start();
    let user_routes = rocket::routes![
        app::routes::get_all, app::routes::find_user,
        app::routes::delete_user_by_id, app::routes::create_new_user];
    let catchers = rocket::catchers![app::catchers::internal_error, app::catchers::not_found];
    rocket::ignite()
        .mount("/users", user_routes)
        .register(catchers)
        .launch();
}