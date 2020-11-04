use crate::domain::models::User;
use crate::adapter::outbound::persistence::models::UserData;

pub fn map_to_domain(user_data: UserData) -> User {
    User {
        id: Some(user_data.id),
        first_name: user_data.first_name,
        last_name: user_data.last_name,
        phone: user_data.phone,
        active: user_data.active,
    }
}