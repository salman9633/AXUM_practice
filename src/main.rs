use axum::extract::{Path, Query, Request, State};
use axum::routing::post;
use axum::{body::Body, response::Response};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[tokio::main]
async fn main() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server Listening to {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let shared_state = Arc::new(Mutex::new(Person { name: "JOHN".to_string(), age: 19 }));
    Router::new()
        .route("/", get(|| async { "Landing page" }))
        .route("/home/{id}", get(home))
        .route("/about", get(about))
        .route("/create-user", post(create_user))
        .route("/json-response", get(json_response))
        .route("/response-type", get(response_type))
        .route("/into-response-test", get(into_repose_impl))
        .route("/share1", get(share1))
        .route("/share2", get(share2)).with_state(shared_state)
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
    let person = Person {
        name: "Salman".to_string(),
        age: 25,
    };
    let json_data = to_string_pretty(&person).unwrap();
    Response::new(Body::new(json_data))
}

async fn into_repose_impl() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Completed".to_string())
}

async fn share1(State(person): State<Arc<Mutex<Person>>>) -> impl IntoResponse {
    let mut person = person.lock().unwrap();
    println!("{:?}", person);
    (*person).name = "Tom".to_string();
    (*person).age = 13;
    (StatusCode::ACCEPTED, "Changed".to_string())
}

async fn share2(State(person): State<Arc<Mutex<Person>>>) -> String {
    println!("{:?}",person);

    "SHared2".to_string()
}

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Deserialize)]
struct PersonRequest {
    name: String,
    age: i32,
}
