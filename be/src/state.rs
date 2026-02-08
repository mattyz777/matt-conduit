use sea_orm::DatabaseConnection;
use deadpool_redis::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_pool: Pool,
}