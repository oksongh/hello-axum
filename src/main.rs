// Copyright (c) 2021 Axum Tutorial Contributors
// SPDX-License-Identifier: MIT
// https://github.com/programatik29/axum-tutorial/
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}
