use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_macros::debug_handler;
use uuid::Uuid;

use crate::model::{CreateTodo, SharedState, Todo};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route(
            "/todo",
            post(create_todo_handler).get(get_todo_list_handler),
        )
        .route(
            "/todo/:id",
            get(get_todo_handler)
        )
}

pub async fn get_todo_list_handler(
    State(state): State<SharedState>
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let todos: Vec<Todo> = sqlx::query_as!(
        Todo,
        "SELECT id, title, content, completed, created_at FROM todo"
    )
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todos))
}

pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, title, content, completed, created_at FROM todo WHERE id=$1",
        id
    )
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match todo {
        Some(value) => Ok(Json(value)),
        _ => Err((StatusCode::NOT_FOUND, format!("Todo(id={}) not found", id)))
    }
}

#[debug_handler]
pub async fn create_todo_handler(
    State(state): State<SharedState>,
    Json(body): Json<CreateTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
            INSERT INTO todo(title, content)
            VALUES ($1, $2)
            RETURNING id, title, content, completed, created_at
        "#,
        body.title,
        body.content
    )
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            let error = format!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, error)
        } )?;

    Ok(Json(todo))
}