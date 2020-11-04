use crate::domain::models::User;
use crate::adapter::outbound::persistence::write_adapter;

pub fn create_new_user(user: User) -> User {
    write_adapter::create_user(user)
}

pub fn delete_existing_user(user_id: i32) {
    write_adapter::delete_user_by_id(user_id);
}