use axum::{Json, Router, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello-world", get(hello_world));

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
