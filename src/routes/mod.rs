use axum::Router;
use axum::routing::{get, post};

use crate::model::SharedState;
use crate::routes::health::get_health_handler;
use crate::routes::todo::{create_todo_handler, get_todo_handler};
use crate::routes::todo::get_todo_list_handler;

mod health;
mod todo;

pub fn get_routes() -> Router<SharedState> {
    Router::new()
        .route(
            "/todo",
            post(create_todo_handler).get(get_todo_list_handler),
        )
        .route("/todo/:id", get(get_todo_handler))
        .route("/health/info", get(get_health_handler))
}