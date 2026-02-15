use axum::Router;
use axum::routing::{get, post, put, delete};
use crate::state::AppState;
use crate::observability::http::with_request_id;
use crate::handler::user_handler;

pub fn init_router(state: AppState) -> Router {
    let api_router = Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users/{id}", get(user_handler::get_user))
        .route("/users/{id}", put(user_handler::update_user))
        .route("/users/{id}", delete(user_handler::delete_user))
        .route("/login", post(user_handler::login));

    let router = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .nest("/api", api_router)
        .with_state(state);

    with_request_id(router)
}
