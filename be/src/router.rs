
use axum::Router;
use axum::routing::{get, post, delete, put};
use crate::state::AppState;
use crate::observability::http::with_request_id;

pub fn init_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .with_state(state);

    with_request_id(router)
}