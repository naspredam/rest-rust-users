use dotenv::dotenv;

use std::env;
use std::result::Result;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
static mut CONNECTION_POOL: Option<MysqlPool> = None;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().max_size(10).build(manager)
}

pub fn start_connection_pool() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool_result = init(&database_url);
    unsafe {
        CONNECTION_POOL = match connection_pool_result {
            Ok(pool) => Some(pool),
            Err(error) => panic!("Unable to connect to the database due to: {:?}", error),
        };
    }
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
