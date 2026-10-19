use axum::{Router, routing::get};

mod health;
mod tasks;
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health::check))
        .route("/tasks", get(tasks::list).post(tasks::create))
}
