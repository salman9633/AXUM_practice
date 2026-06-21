use axum::extract::{Path, Query};
use axum::routing::post;
use axum::{routing::get, Json, Router};
use std::collections::HashMap;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server Listening to {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Landing page" }))
        .route("/home/{id}", get(home))
        .route("/about", get(about))
        .route("/create-user", post(create_user))
}

async fn home(Path(id): Path<i32>) -> String {
    id.to_string()
}

async fn about(Query(params): Query<HashMap<String, String>>) -> &'static str {
    for k in params.keys() {
        println!("{}", k)
    }
    for k in params.values() {
        println!("{}", k)
    }

    for (k, v) in params.iter() {
        println!("{}:{}", k, v)
    }

    "About"
}

async fn create_user(Json(person): Json<PersonRequest>)->&'static str {
    println!("{:?}", person);

    "user Created"
}

#[derive(Debug, Deserialize)]
struct PersonRequest {
    name: String,
    age: i32,
}
