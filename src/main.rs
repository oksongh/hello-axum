// Copyright (c) 2021 Axum Tutorial Contributors
// SPDX-License-Identifier: MIT
// https://github.com/programatik29/axum-tutorial/
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(todos_index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn todos_index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}
