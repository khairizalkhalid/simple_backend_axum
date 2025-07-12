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

pub async fn hello_world_options(
    db_conn: extract::State<Arc<Mutex<Connection>>>,
    Path(opt): Path<i8>) -> Json<HelloWorldResponse> {
    let response = hello_world::get_message_for_option(&db_conn, opt).unwrap();
    Json(response)
}

#[axum::debug_handler]
pub async fn hello_world_submit(
    db_conn: extract::State<Arc<Mutex<Connection>>>,
    Json(request): Json<HelloWorldRequest>,
) -> Json<HelloWorldResponse> {
    let response = hello_world::save_message(&db_conn, request).unwrap();
    Json(HelloWorldResponse { message: response })
}

#[axum::debug_handler]
pub async fn hallow_world(
    db_conn: extract::State<Arc<Mutex<Connection>>>,
    Path(opt): Path<i8>,
) -> Json<HelloWorldResponse> {
    let response = hello_world::delete_message(&db_conn, opt).unwrap();
    Json(HelloWorldResponse { message: response })
}
