use std::sync::{Arc, Mutex};

use axum::{
    Json,
    extract::{self, Path},
};
use rusqlite::{Connection, params};

use crate::dto::hello_world::{HelloWorldRequest, HelloWorldResponse};

pub async fn hello_world() -> Json<HelloWorldResponse> {
    let resp = HelloWorldResponse {
        message: "Hello World".to_string(),
    };
    Json(resp)
}

pub async fn hello_world_options(Path(opt): Path<i8>) -> Json<HelloWorldResponse> {
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
pub async fn hello_world_submit(
    db_conn: extract::State<Arc<Mutex<Connection>>>,
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
