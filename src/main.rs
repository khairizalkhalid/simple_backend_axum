mod config;

use std::{
    i8,
    sync::{Arc, Mutex},
};

use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let db_conn = config::sqllite::connect();
    config::sqllite::init_db(&db_conn);

    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/hello-world/{opt}", get(hello_world_options))
        .route("/hello-world-submit", post(hello_world_submit))
        .with_state(db_conn);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Start listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct HelloWorldRequest {
    name: String,
    message: String,
}

#[derive(Serialize)]
struct HelloWorldResponse {
    message: String,
}

async fn hello_world() -> Json<HelloWorldResponse> {
    let resp = HelloWorldResponse {
        message: "Hello World".to_string(),
    };
    Json(resp)
}

async fn hello_world_options(Path(opt): Path<i8>) -> Json<HelloWorldResponse> {
    let resp = match opt {
        0 => HelloWorldResponse {
            message: "ゼロ".to_string(),
        },
        1 => HelloWorldResponse {
            message: "Itchy".to_string(),
        },
        _ => HelloWorldResponse {
            message: "-_-".to_string(),
        },
    };

    Json(resp)
}

#[axum::debug_handler]
async fn hello_world_submit(
    db_conn: axum::extract::State<Arc<Mutex<Connection>>>,
    Json(request): Json<HelloWorldRequest>,
) -> Json<HelloWorldResponse> {
    let response = format!("Hello World from {}! \"{}\"", request.name, request.message);

    {
        let db_conn = db_conn.lock().unwrap();
        db_conn
            .execute(
                "INSERT INTO hello_world_messages (name, message) VALUES (?1, ?2)",
                params![request.name, request.message],
            )
            .unwrap();
    }

    Json(HelloWorldResponse { message: response })
}
