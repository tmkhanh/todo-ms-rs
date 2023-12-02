use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: String,
    pub content: String
}

#[derive(Default)]
pub struct AppState {
    pub todos: Vec<Todo>,
}

pub type SharedState = Arc<Mutex<AppState>>;