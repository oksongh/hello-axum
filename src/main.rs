// Copyright (c) 2021 Axum Tutorial Contributors
// SPDX-License-Identifier: MIT
// https://github.com/tokio-rs/axum/tree/main/examples/todos

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, patch},
    BoxError, Json, Router,
};
use memory_db::DB;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect(".env not found");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_sandbox=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = memory_db::DB::default();
    let app = Router::new()
        .route("/", get(todos_index))
        .route("/todos", get(todos_read).post(todos_create))
        .route("/todos/:id", patch(todos_update).delete(todos_delete))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn todos_index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}
#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}
async fn todos_create(State(db): State<DB>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo::new(input.text);
    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    state: Option<TaskState>,
}
async fn todos_update(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }
    if let Some(state) = input.state {
        todo.state = state;
    }

    db.write().unwrap().insert(todo.id, todo.clone());
    Ok(Json(todo))
}

async fn todos_delete(Path(id): Path<Uuid>, State(db): State<DB>) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn todos_read(State(db): State<DB>) -> impl IntoResponse {
    let todos = db.read().unwrap();
    let todos = todos.values().cloned().collect::<Vec<Todo>>();
    Json(todos)
}
mod memory_db {
    use std::{collections::HashMap, sync::Arc};

    use uuid::Uuid;

    use crate::Todo;

    pub type DB = Arc<std::sync::RwLock<HashMap<Uuid, Todo>>>;
}
#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String,
    state: TaskState,
}
impl Todo {
    fn new(text: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            text,
            state: TaskState::New,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
enum TaskState {
    New,
    Going,
    Done,
}
