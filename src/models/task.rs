use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
