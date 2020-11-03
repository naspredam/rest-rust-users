use crate::application::mapper;
use crate::application::models::{ UserDto, UserChangeBodyDto };
use crate::domain::models::User;
use crate::infrastructure::repository;

use rocket::{delete, get, post};
use rocket_contrib::json;

use std::option::Option;

#[get("/", format = "application/json")]
pub fn get_all() -> json::Json<json::JsonValue> {
    let users: Vec<UserDto> = repository::find_all_users()
        .into_iter()
        .map(mapper::map_to_dto)
        .rev()
        .collect();

    json::Json(json!(users))
}

#[post("/", format = "application/json", data = "<new_user>")]
pub fn create_new_user(new_user: json::Json<UserChangeBodyDto>) -> json::Json<json::JsonValue> {
    let user: User = mapper::map_to_domain(new_user.into_inner());
    let saved_user: User = repository::create_user(user);
    json::Json(json!(mapper::map_to_dto(saved_user)))
}

#[get("/<user_id>", format = "application/json")]
pub fn find_user(user_id: i32) -> Option<json::Json<json::JsonValue>> {
    repository::find_user_by_id(user_id)
        .map(mapper::map_to_dto)
        .map(|user_dto| json::Json(json!(user_dto)))
}

#[delete("/<user_id>", format = "application/json")]
pub fn delete_user_by_id(user_id: i32) {
    repository::delete_user_by_id(user_id);
}
