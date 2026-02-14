use be::observability::tracing::init_tracing;
use be::{get_db_connection, get_redis_pool};
use be::AppState;
use anyhow::Result;
use be::init_router;
use tokio::signal;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    
    if let Err(e) = run().await {
        error!(error = %e, "Application failed to start");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    info!("Starting application...");
    
    let db = get_db_connection().await?;
    let redis_pool = get_redis_pool().await?;
    let state = AppState { db, redis_pool };

    let app = init_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}