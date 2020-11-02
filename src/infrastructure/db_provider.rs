use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::result::Result;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
static mut CONNECTION_POOL: Option<MysqlPool> = None;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().max_size(4).build(manager)
}

pub fn start_connection_pool() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection_pool_result = init(&database_url);
    unsafe {
        CONNECTION_POOL = match connection_pool_result {
            Ok(pool) => Some(pool),
            Err(..) => None
        };
    }
}

pub fn stablish_connection() -> MySqlPooledConnection {
    unsafe {
        CONNECTION_POOL
            .clone()
            .expect("Connection pool...")
            .get()
            .map_err(|err| { println!( "get connection from pool error in line:{} ! error: {:?}", line!(), err) })
            .expect("Retrieve the connection...")
    }
}

pub fn connect() -> MysqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    init(&database_url)
        .expect("Error on starting the pool")
}
