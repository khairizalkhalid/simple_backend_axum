use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn connect() -> Arc<Mutex<Connection>> {
    let path = "./axum.test.db"; // TODO: move this to config files
    Arc::new(Mutex::new(Connection::open(path).unwrap()))
}

pub fn init_db(db_conn: &Arc<Mutex<Connection>>) {
    let conn = db_conn.lock().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS hello_world_messages ( 
               id INTEGER PRIMARY KEY, 
               name TEXT NOT NULL, 
               message TEXT NOT NULL
           )",
        [],
    )
    .unwrap();
}
