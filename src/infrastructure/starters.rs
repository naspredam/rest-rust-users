use crate::infrastructure::db_provider;

pub fn start() {
    db_provider::start_connection_pool();
}
