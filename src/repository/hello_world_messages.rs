use rusqlite::{Connection, params};

pub fn save_message(conn: &Connection, name: &str, message: &str) {
    conn.execute(
        "INSERT INTO hello_world_messages (name, message) VALUES (?1, ?2)",
        params![name, message],
    )
    .unwrap();
}
