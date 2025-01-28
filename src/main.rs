// Copyright (c) 2021 Axum Tutorial Contributors
// SPDX-License-Identifier: MIT
// https://github.com/tokio-rs/axum/tree/main/examples/todos

use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::Html, routing::get, BoxError,
    Router,
};
use serde::Serialize;
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

mod memory_db {
    use std::{collections::HashMap, sync::Arc};

    use uuid::Uuid;

    use crate::Todo;

    pub type DB = Arc<std::sync::RwLock<HashMap<Uuid, Todo>>>;
    // impl DB {
    //     fn read(&self, id: Uuid) -> Result<&Todo, &str> {
    //         self.read()?.get(&id).ok_or("not found")
    //     }
    // }
}
#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String,
    state: TaskState,
}

#[derive(Debug, Serialize, Clone, Default)]
enum TaskState {
    #[default]
    New,
    Going,
    Done,
}
impl TaskState {
    fn transition(&mut self) {
        *self = match &self {
            Self::New => Self::Going,
            Self::Going => Self::Done,
            Self::Done => Self::Done,
        };
    }
}
