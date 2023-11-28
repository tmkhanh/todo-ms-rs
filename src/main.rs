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

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    use super::*;

    fn send_get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn get_health_info() {
        let app = get_routes();

        let response = app.oneshot(send_get_request("/health/info")).await.unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "status": "UP" }));
    }
}