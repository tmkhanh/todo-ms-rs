#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::StatusCode;
    use chrono::prelude::*;
    use serde_json::json;
    use tower::ServiceExt;

    use todo::model::{CreateTodo, Todo};

    use crate::common::{get_default_app, get_response_body_value, get_state, send_get_request, send_post_request};

    async fn try_create_todo() -> (CreateTodo, Todo) {
        let millis = Utc::now().timestamp_millis();
        let create_todo = CreateTodo {
            title: format!("test title {}", millis),
            content: format!("test content {}", millis),
        };
        let post_body = json!(create_todo);

        let router = get_default_app(&get_state()).await;
        let response = (router)
            .oneshot(send_post_request("/todo", Body::from(
                post_body.to_string()
            )))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        let body = get_response_body_value(response).await;
        let todo: Todo = serde_json::from_value(body).unwrap();

        (create_todo, todo)
    }

    #[tokio::test]
    async fn create_todo() {
        let (create_todo, todo) = try_create_todo().await;

        assert_eq!(todo.title, create_todo.title);
        assert_eq!(todo.content, create_todo.content);
    }

    #[tokio::test]
    async fn list_todo() {
        let initial_state = get_state();
        let router = get_default_app(&initial_state).await;
        let response = router
            .oneshot(send_get_request("/todo"))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        let body = get_response_body_value(response).await;
        let todos: Vec<Todo> = serde_json::from_value(body).unwrap();

        assert_ne!(todos.len(), 0);

        let actual = todos.get(0).unwrap();
        let expected = initial_state.lock().unwrap().todos.get(0).unwrap().clone();
        assert_eq!(*actual, expected);
    }

    #[tokio::test]
    async fn get_todo() {
        let initial_state = get_state();
        let expected = initial_state.lock().unwrap().todos.get(0).unwrap().clone();

        let router = get_default_app(&initial_state).await;
        let response = router
            .oneshot(send_get_request(&format!("/todo/{}", expected.id)))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        let body = get_response_body_value(response).await;
        let todo: Todo = serde_json::from_value(body).unwrap();

        assert_eq!(todo, expected);
    }
}