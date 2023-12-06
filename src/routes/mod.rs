use axum::Router;

use crate::model::SharedState;

mod health;
mod todo;

pub fn get_routes() -> Router<SharedState> {
    Router::new()
        .merge(todo::router())
        .merge(health::router())
}