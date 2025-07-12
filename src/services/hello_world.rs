use std::sync::{Arc, Mutex};

use crate::{
    dto::hello_world::{HelloWorldRequest, HelloWorldResponse},
    repository::hello_world_messages,
};

pub fn get_hellow_world() -> HelloWorldResponse {
    HelloWorldResponse {
        message: "Hello World".to_string(),
    }
}

pub fn get_message_for_option(
    db_conn: &Arc<Mutex<rusqlite::Connection>>,
    opt: i8,
) -> rusqlite::Result<HelloWorldResponse> {
    let conn = db_conn.lock().unwrap();
    let response = hello_world_messages::get_messages(&conn, &opt);

    Ok(response)
}

pub fn save_message(
    db_conn: &Arc<Mutex<rusqlite::Connection>>,
    request: HelloWorldRequest,
) -> rusqlite::Result<String> {
    let response = format!("Hello World from {}! \"{}\"", request.name, request.message);

    let conn = db_conn.lock().unwrap();

    hello_world_messages::save_message(&conn, &request.name, &request.message);

    Ok(response)
}
