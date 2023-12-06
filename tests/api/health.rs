#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::common::{get_default_app, get_response_body_value, send_get_request};

    #[tokio::test]
    async fn get_health_info() {
        let app = get_default_app().await;
        let response = app
            .oneshot(send_get_request("/health/info"))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(get_response_body_value(response).await, json!({ "status": "UP" }));
    }
}