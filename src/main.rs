use axum::extract::Path;
use axum::{routing::get, Router};

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
}

async fn home(Path(id): Path<i32>) -> String {
    id.to_string()
}

async fn about() -> &'static str {
    "About"
}
