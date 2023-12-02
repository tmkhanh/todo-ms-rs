use axum::Json;
use axum::response::IntoResponse;

pub async fn get_health_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "UP"
    });

    Json(json_response)
}