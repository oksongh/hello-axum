// Copyright (c) 2021 Axum Tutorial Contributors
// SPDX-License-Identifier: MIT
// https://github.com/programatik29/axum-tutorial/
use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
#[derive(Deserialize)]
struct RangeParams {
    start: usize,
    end: usize,
}
async fn handler(Query(range): Query<RangeParams>) -> Html<String> {
    let random_num = thread_rng().gen_range(range.start..range.end);
    println!("hey:{random_num}");
    Html(format!("<h1>Random number: {random_num}</h1>"))
}
