use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Task {
    pub id: u64,
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
