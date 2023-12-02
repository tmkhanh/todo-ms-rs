use std::sync::{Arc, Mutex};
use todo::model::AppState;
use todo::routes::get_routes;

#[tokio::main]
async fn main() {
    let app = get_routes()
        .with_state(Arc::new(Mutex::new(AppState { todos: Vec::new() })));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}