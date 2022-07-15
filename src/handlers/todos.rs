use axum::{Json, Router};
use axum::routing::{get, post};
use chrono::Utc;
use futures::StreamExt;
use mongodb::results::InsertOneResult;
use crate::mongo::Todo;
use crate::types::{ApiResult, SharedTodos};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new()
        .route("/api/todos", get(get_todos))
        .route("/api/todo", post(add_todo))
}

async fn get_todos(
    todos: SharedTodos
) -> ApiResult<Json<Vec<Todo>>> {
    let mut cursors = todos.find(None, None).await?;

    let mut res: Vec<Todo> = Vec::new();

    while let Some(todo) = cursors.next().await {
        res.push(todo?)
    }

    Ok(Json(res))
}

#[derive(Deserialize)]
struct TodoAdd {
    content: String,
}

async fn add_todo(
    todos: SharedTodos,
    Json(TodoAdd { content }): Json<TodoAdd>,
) -> ApiResult<Json<InsertOneResult>> {
    let todo = Todo {
        creator: "".to_string(),
        content,
        create_at: Utc::now(),
    };
    let res = todos.insert_one(todo, None).await?;

    Ok(Json(res))
}

async fn complete_todo() {}