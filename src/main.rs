// Copyright (c) 2021 Axum Tutorial Contributors
// SPDX-License-Identifier: MIT
// https://github.com/programatik29/axum-tutorial/
use axum::{response::Html, routing::get, Router};
use serde::Serialize;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = memory_db::DB::default();
    let app = Router::new().route("/", get(todos_index)).with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
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
