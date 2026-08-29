use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn init_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let max_connections = env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(10);

    let acquire_timeout = env::var("DB_ACQUIRE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(10);

    PgPoolOptions::new()
        .max_connections(max_connections)
        // Without a bound, a saturated pool makes requests hang instead of
        // failing fast with a 500.
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .connect(&database_url)
        .await
}
