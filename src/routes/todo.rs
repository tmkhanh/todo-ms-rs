use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum_macros::debug_handler;
use uuid::Uuid;

use crate::model::{CreateTodo, SharedState, Todo};

pub async fn get_todo_list_handler(
    State(state): State<SharedState>
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let json_response = &state.lock().unwrap().todos;

    Ok(Json(json_response.clone()))
}

pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todos = &state.lock().unwrap().todos;
    let todo = todos.iter()
        .filter(|i| i.id == id)
        .next();

    match todo {
        Some(value) => {
            let todo1: Todo = (*value).clone();
            Ok(Json(todo1))
        }
        _ => Err((StatusCode::NOT_FOUND, format!("Todo(id={}) not found", id)))
    }
}

#[debug_handler]
pub async fn create_todo_handler(
    State(state): State<SharedState>,
    Json(body): Json<CreateTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let todo = Todo {
        id: Uuid::new_v4(),
        title: body.title,
        content: body.content,
        created_at: chrono::Utc::now(),
        completed: false,
    };
    state.lock().unwrap().todos.push(todo.clone());
    Ok(Json(todo))
}