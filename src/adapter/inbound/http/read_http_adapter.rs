use crate::application::use_case::find_user_use_case;
use crate::adapter::inbound::http::mapper;
use crate::adapter::inbound::http::models::UserDto;

pub fn fetch_all_users() -> Vec<UserDto> {
    find_user_use_case::fetch_all()
        .into_iter()
        .map(mapper::map_to_dto)
        .rev()
        .collect()
}

pub fn fetch_user_by_id(user_id: i32) -> Option<UserDto> {
    find_user_use_case::fetch_by_id(user_id)
        .map(mapper::map_to_dto)
}