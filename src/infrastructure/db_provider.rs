use dotenv::dotenv;

use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
static mut CONNECTION_POOL: Option<MysqlPool> = None;

fn init_connection_pool() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let connection_pool_result = Pool::builder().max_size(10).build(manager);
    unsafe {
        CONNECTION_POOL = match connection_pool_result {
            Ok(pool) => Some(pool),
            Err(error) => panic!("Unable to connect to the database due to: {:?}", error),
        };
    }
}

fn run_migrations() {
    diesel_migrations::embed_migrations!("migrations/");
    embedded_migrations::run(&stablish_connection()).unwrap();
}

pub fn connect_to_datbase() {
    dotenv().ok();
    init_connection_pool();
    run_migrations();
}

type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;
pub fn stablish_connection() -> MySqlPooledConnection {
    unsafe {
        CONNECTION_POOL
            .clone()
            .expect("Not able to get the connection...")
            .get()
            .map_err(|err| { println!( "get connection from pool error in line:{} ! error: {:?}", line!(), err) })
            .expect("Retrieve the connection...")
    }
}
