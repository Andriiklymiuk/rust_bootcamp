use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Status {
    status: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/status", get(status))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn status() -> Json<Status> {
    let status = Status {
        // Bug 1: check type
        status: "running",
        version: "1.0.0",
    };
    Json(status)
}

async fn create_user(Json(user): Json<User>) -> Json<User> {
    // Bug 2: cannot print
    println!("{:?}", user);
    Json(user)
}
