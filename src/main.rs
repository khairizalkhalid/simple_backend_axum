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
struct HelloWorlResp {
    message: String,
}

async fn hello_world() -> Json<HelloWorlResp> {
    let resp = HelloWorlResp {
        message: "Hello World".to_string(),
    };
    Json(resp)
}

async fn hello_world_options(Path(opt): Path<i8>) -> Json<HelloWorlResp> {
    let resp = match opt {
        0 => HelloWorlResp {
            message: "ゼロ".to_string(),
        },
        1 => HelloWorlResp {
            message: "Itchy".to_string(),
        },
        _ => HelloWorlResp {
            message: "-_-".to_string(),
        },
    };

    Json(resp)
}
