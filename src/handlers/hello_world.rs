use std::sync::{Arc, Mutex};

use axum::{
    Json,
    extract::{self, Path},
};
use rusqlite::Connection;

use crate::{
    dto::hello_world::{HelloWorldRequest, HelloWorldResponse},
    services::hello_world,
};

pub async fn hello_world() -> Json<HelloWorldResponse> {
    Json(hello_world::get_hellow_world())
}

pub async fn hello_world_options(Path(opt): Path<i8>) -> Json<HelloWorldResponse> {
    Json(hello_world::get_message_for_option(opt))
}

#[axum::debug_handler]
pub async fn hello_world_submit(
    db_conn: extract::State<Arc<Mutex<Connection>>>,
    Json(request): Json<HelloWorldRequest>,
) -> Json<HelloWorldResponse> {
    let response = hello_world::save_message(&db_conn, request).unwrap();
    Json(HelloWorldResponse { message: response })
}
