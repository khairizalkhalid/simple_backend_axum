mod config;
mod dto;
mod handlers;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::hello_world::{hello_world, hello_world_options, hello_world_submit};

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
