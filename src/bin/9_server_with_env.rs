use axum::{extract::State, response::Json, routing::get, Router};
use dotenv::from_path;
use serde::Serialize;
use std::{env, path::Path, sync::Arc};

struct EnvVariables {
    port: String,
    app_version: String,
    // Task 1: Add environment field
}

impl EnvVariables {
    fn load() -> Self {
        from_path(Path::new(".env-server")).ok();

        Self {
            port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
            app_version: env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
        }
    }
}

#[derive(Serialize)]
struct Status {
    status: String,
    version: String,
    // Task 2: Add environment field
}

#[tokio::main]
async fn main() {
    let env_vars = Arc::new(EnvVariables::load());
    let addr = format!("127.0.0.1:{}", env_vars.port);

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/status", get(status))
        .with_state(env_vars);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn status(State(env_vars): State<Arc<EnvVariables>>) -> Json<Status> {
    let status = Status {
        status: "running".to_string(),
        version: env_vars.app_version.clone(),
        // Tag 3: Add environment field
    };
    Json(status)
}

