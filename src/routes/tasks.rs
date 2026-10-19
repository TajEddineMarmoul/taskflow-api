use axum::{Json, http::StatusCode};

use crate::models::task::{CreateTask, ErrorResponse, Task};

pub async fn list() -> Json<Vec<Task>> {
    let tasks = vec![
        Task {
            id: 1,
            title: "Learn Axum".to_string(),
        },
        Task {
            id: 2,
            title: "Build a backend".to_string(),
        },
    ];

    Json(tasks)
}

pub async fn create(
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, Json<ErrorResponse>)> {
    if payload.title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Empty title".to_string(),
            }),
        ));
    }

    let task = Task {
        id: 3,
        title: payload.title,
    };

    Ok((StatusCode::CREATED, Json(task)))
}
