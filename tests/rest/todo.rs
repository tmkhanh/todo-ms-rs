#[cfg(test)]
mod todo_tests {
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::Router;
    use chrono::Utc;
    use serde_json::json;
    use sqlx::PgPool;
    use tower::ServiceExt;
    use std::borrow::BorrowMut;
    use uuid::Uuid;

    use todo::model::{CreateTodo, Todo};

    use crate::common::{app_with_pool, send_get_request, get_response_body_value, send_post_request};

    async fn try_create_todo(router: &mut Router) -> (CreateTodo, Todo) {
        let millis = Utc::now().timestamp_millis();
        let create_todo = CreateTodo {
            title: format!("test title {}", millis),
            content: format!("test content {}", millis),
        };
        let post_body = json!(create_todo);

        let response = router
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

    #[sqlx::test(fixtures(path = "../fixtures", scripts("todo")))]
    async fn create_todo(pool: PgPool) {
        let mut router = app_with_pool(pool).await;
        let (create_todo, todo) = try_create_todo(&mut router).await;

        assert_eq!(todo.title, create_todo.title);
        assert_eq!(todo.content, create_todo.content);

        // Bad request error
        let post_body = json!({ "title": "" });

        let response = router
            .oneshot(send_post_request("/todo", Body::from(
                post_body.to_string()
            )))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("todo")))]
    async fn list_todo(pool: PgPool) {
        let router = app_with_pool(pool).await;
        let response = router
            .oneshot(send_get_request("/todo"))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        let body = get_response_body_value(response).await;
        let todos: Vec<Todo> = serde_json::from_value(body).unwrap();

        assert_eq!(todos.len(), 2);
    }

    #[sqlx::test(fixtures(path = "../fixtures", scripts("todo")))]
    async fn get_todo(pool: PgPool) {
        let mut router = app_with_pool(pool).await;
        let (_, todo) = try_create_todo(&mut router).await;

        // Check the response status code.
        let response = router
            .borrow_mut()
            .oneshot(send_get_request(&format!("/todo/{}", todo.id)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = get_response_body_value(response).await;
        let retrieved_todo: Todo = serde_json::from_value(body).unwrap();
        assert_eq!(retrieved_todo, todo);

        // NotFound error
        let response = router
            .borrow_mut()
            .oneshot(send_get_request(&format!("/todo/{}", Uuid::new_v4())))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

