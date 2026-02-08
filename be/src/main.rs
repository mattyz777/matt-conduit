use axum::{Json, Router, routing::get};
use be::observability::tracing::init_tracing;
use be::{get_db_connection, get_redis_pool};
use be::AppState;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();
    
    let db = get_db_connection().await?;
    let redis_pool = get_redis_pool().await?;
    let _state = AppState { db, redis_pool };
    Ok(())
}
