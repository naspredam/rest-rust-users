use crate::domain::models::User;
use crate::adapter::outbound::persistence::read_adapter;

use std::option::Option;

pub fn fetch_all() -> Vec<User> {
    read_adapter::find_all_users()
}

pub fn fetch_by_id(user_id: i32) -> Option<User> {
    read_adapter::find_user_by_id(user_id)
}
