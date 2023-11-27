use axum::{Json, Router, routing::get};
use axum::response::IntoResponse;

pub async fn health_checker_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "UP"
    });

    Json(json_response)
}

fn get_routes() -> Router {
    Router::new()
        .route("/health/info", get(health_checker_handler))
}

#[tokio::main]
async fn main() {
    let app = get_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}