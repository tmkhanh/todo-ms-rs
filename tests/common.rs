use std::sync::{Arc, Mutex};

use axum::{http, Router};
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use chrono::Utc;
use http_body_util::BodyExt;
use serde_json::Value;
use uuid::Uuid;

use todo::model::{AppState, SharedState, Todo};
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

pub fn get_state() -> SharedState {
    let mut todos = Vec::new();
    todos.push(Todo {
        id: Uuid::new_v4(),
        title: "title".to_string(),
        content: "content".to_string(),
        completed: true,
        created_at: Utc::now(),
    });
    Arc::new(Mutex::new(AppState { todos }))
}

pub async fn get_default_app(state: &SharedState) -> Router {
    get_routes()
        .with_state(state.clone())
}

pub async fn get_response_body_value(response: Response) -> Value {
    let body = (response).into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice::<Value>(&body).unwrap()
}