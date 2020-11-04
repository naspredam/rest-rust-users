use crate::application::use_case::persist_user_use_case;
use crate::adapter::inbound::http::mapper;
use crate::adapter::inbound::http::models::{UserDto, UserChangeBodyDto};
use crate::domain::models::User;

pub fn create_new_user(user_change_body_dto: UserChangeBodyDto) -> UserDto {
    let user: User = mapper::map_to_domain(user_change_body_dto);
    let saved_user: User = persist_user_use_case::create_new_user(user);
    mapper::map_to_dto(saved_user)
}

pub fn delete_user(user_id: i32) {
    persist_user_use_case::delete_existing_user(user_id);
}