use axum::{Router, routing::get};
use sqlx::PgPool;

mod health;
mod tasks;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health::check))
        .route("/tasks", get(tasks::list).post(tasks::create))
        .with_state(pool)
}
