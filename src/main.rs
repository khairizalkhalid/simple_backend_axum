use std::i8;

use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/hello-world/{opt}", get(hello_world_options));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Start listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
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

async fn hello_world_submit(Json(HellowWorldReq
