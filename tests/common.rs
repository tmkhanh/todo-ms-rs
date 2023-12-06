use axum::{http, Router};
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::PgPool;

use todo::configuration::{get_connection_pool, load_configuration};
use todo::model::AppState;
use todo::routes::get_routes;

pub fn send_get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .method("GET")
        .body(Body::empty())
        .unwrap()
}

pub fn send_post_request(uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .method("POST")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(body)
        .unwrap()
}

pub async fn get_default_app() -> Router {
    let configuration = load_configuration().expect("Failed to read configuration.");
    let connection_pool = get_connection_pool(&configuration.database);
    app_with_pool(connection_pool).await
}

pub async fn app_with_pool(pool: PgPool) -> Router {
    let state = AppState { db: pool };
    get_routes()
        .with_state(state.clone())
}

pub async fn get_response_body_value(response: Response) -> Value {
    let body = (response).into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice::<Value>(&body).unwrap()
}