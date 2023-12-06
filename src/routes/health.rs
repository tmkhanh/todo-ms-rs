use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;

use crate::model::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/health/info", get(get_health_handler))
}

pub async fn get_health_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "UP"
    });

    Json(json_response)
}