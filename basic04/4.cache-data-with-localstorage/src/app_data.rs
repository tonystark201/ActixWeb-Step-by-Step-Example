use std::sync::Arc;
use diesel::PgConnection;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

// #[derive(Clone)]
pub struct AppData {
    pub pool: Arc<Pool<RedisConnectionManager>>,
    pub db_conn: PgConnection
}

