use crate::domain::models::User;
use crate::adapter::inbound::http::models::{UserDto, UserChangeBodyDto};

pub fn map_to_dto(user: User) -> UserDto {
    UserDto {
        id: user.id.unwrap(),
        first_name: user.first_name,
        last_name: user.last_name,
        phone: user.phone,
    }
}

pub fn map_to_domain_with_id(user_id: i32, user_change_body_dto: UserChangeBodyDto) -> User {
    User {
        id: Some(user_id),
        first_name: user_change_body_dto.first_name,
        last_name: user_change_body_dto.last_name,
        phone: user_change_body_dto.phone,
    }
}

pub fn map_to_domain(user_change_body_dto: UserChangeBodyDto) -> User {
    User {
        id: None,
        first_name: user_change_body_dto.first_name,
        last_name: user_change_body_dto.last_name,
        phone: user_change_body_dto.phone,
    }
}