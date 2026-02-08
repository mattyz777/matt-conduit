use sea_orm::{Database, DatabaseConnection};
use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
use tracing::info;
use std::env;
use anyhow::Result;

pub async fn get_db_connection() -> Result<DatabaseConnection> {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db  = Database::connect(database_url).await?;
    info!("Database connected successfully");
    Ok(db)
}

pub async fn get_redis_pool() -> Result<Pool> {
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL must be set in .env file");
    let redis_config = RedisConfig::from_url(redis_url);
    let redis_pool = redis_config.create_pool(Some(Runtime::Tokio1))?;
    info!("Redis connected successfully");
    Ok(redis_pool)
}
