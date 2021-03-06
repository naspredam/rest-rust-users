use crate::adapter::inbound::http::{read_http_adapter, write_http_adapter};
use crate::adapter::inbound::http::models::UserChangeBodyDto;

use rocket::{delete, get, post, put, catch, Request};
use rocket_contrib::json;

#[catch(500)]
fn internal_error() -> json::Json<json::JsonValue> {
    json::Json(json!({
        "message": "Whoops! Looks like we messed up.",
    }))
}

#[catch(404)]
fn not_found(req: &Request) -> json::Json<json::JsonValue> {
    json::Json(json!({
        "message": format!("I couldn't find '{}'...", req.uri()),
    }))
}

#[get("/", format = "application/json")]
fn get_all() -> json::Json<json::JsonValue> {
    let users = read_http_adapter::fetch_all_users();
    json::Json(json!(users))
}

#[get("/<user_id>", format = "application/json")]
fn find_user(user_id: i32) -> Option<json::Json<json::JsonValue>> {
    read_http_adapter::fetch_user_by_id(user_id)
        .map(|user_dto| json::Json(json!(user_dto)))
}

#[post("/", format = "application/json", data = "<new_user>")]
fn create_new_user(new_user: json::Json<UserChangeBodyDto>) -> json::Json<json::JsonValue> {
    let saved_user = write_http_adapter::create_new_user(new_user.into_inner());
    json::Json(json!(saved_user))
}

#[put("/<user_id>", format = "application/json", data = "<user_change_request>")]
fn update_user(user_id: i32, user_change_request: json::Json<UserChangeBodyDto>) -> json::Json<json::JsonValue> {
    let saved_user = write_http_adapter::update_user(user_id, user_change_request.into_inner());
    json::Json(json!(saved_user))
}

#[delete("/<user_id>", format = "application/json")]
fn delete_user_by_id(user_id: i32) {
    write_http_adapter::delete_user(user_id);
}

pub fn launch() {
    let user_routes = rocket::routes![create_new_user, update_user, get_all, find_user, delete_user_by_id];
    let catchers = rocket::catchers![internal_error, not_found];
    rocket::ignite()
        .mount("/users", user_routes)
        .register(catchers)
        .launch();
}