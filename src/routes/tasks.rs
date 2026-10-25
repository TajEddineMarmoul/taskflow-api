use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::models::task::{CreateTask, ErrorResponse, Task};

pub async fn list(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Task>>), (StatusCode, Json<ErrorResponse>)> {
    let tasks = sqlx::query_as::<_, Task>("SELECT id, title FROM tasks ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, Json<ErrorResponse>)> {
    let title = &payload.title.trim();
    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Empty title".to_string(),
            }),
        ));
    }
    let task =
        sqlx::query_as::<_, Task>("INSERT INTO tasks (title) VALUES ($1) RETURNING id, title")
            .bind(title)
            .fetch_one(&pool)
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Database operation failed".to_string(),
                    }),
                )
            })?;

    Ok((StatusCode::CREATED, Json(task)))
}
