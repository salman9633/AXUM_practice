use axum::extract::{Path, Query, Request};
use axum::routing::post;
use axum::{body::Body, response::Response};
use axum::{routing::get, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

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
        .route("/json-response", get(json_response))
        .route("/response-type", get(response_type))
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

async fn create_user(req: Request) -> &'static str {
    // println!("{:?}", person);
    let headers = req.headers();
    let method = req.method();
    let uri = req.uri();

    println!("{:?}", headers);
    println!("{:?}", method);
    println!("{:?}", uri);

    "user Created"
}

async fn json_response() -> Json<Value> {
    Json(json!({
        "name":"SALMAN",
        "age":26
    }))
}

async fn response_type() -> Response {
    Response::new(Body::new("Hello".to_string()))
}

#[derive(Debug, Deserialize)]
struct PersonRequest {
    name: String,
    age: i32,
}
