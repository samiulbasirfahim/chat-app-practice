use redis::Client;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

use super::config::get_config;

pub async fn create_db_pool() -> Result<PgPool, sqlx::Error> {
    let db_uri = get_config().db_url.clone();
    PgPool::connect(db_uri.as_str()).await
}

pub async fn build_redis_client() -> Result<MultiplexedConnection, redis::RedisError> {
    let redis_url = get_config().redis_url.clone();
    let client = Client::open(redis_url)?;
    Ok(client.get_multiplexed_async_connection().await?)
}
