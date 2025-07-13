mod config;
mod dto;
mod handlers;
mod repository;
mod services;

use axum::{
    routing::{delete, get, post}, Router
};
use handlers::hello_world::{hallow_world, hello_world, hello_world_options, hello_world_submit, update_hello_world};

#[tokio::main]
async fn main() {
    let db_conn = config::sqllite::connect();
    config::sqllite::init_db(&db_conn);

    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/hello-world/{opt}", get(hello_world_options))
        .route("/hello-world-submit", post(hello_world_submit))
        .route("/update-hello-world/{id}", post(update_hello_world))
        .route("/hallow-world/{id}", delete(hallow_world))
        .with_state(db_conn);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Start listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
