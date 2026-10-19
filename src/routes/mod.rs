use axum::{Router, routing::get};

mod health;

pub fn create_router() -> Router {
    Router::new().route("/health", get(health::check))
}
