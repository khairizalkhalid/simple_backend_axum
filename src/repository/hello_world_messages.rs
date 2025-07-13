use rusqlite::{Connection, params};

use crate::dto::hello_world::HelloWorldResponse;

pub fn save_message(conn: &Connection, name: &str, message: &str) {
    conn.execute(
        "INSERT INTO hello_world_messages (name, message) VALUES (?1, ?2)",
        params![name, message],
    )
    .unwrap();
}

pub fn update_message(conn: &Connection, id: i8, name: &str, message: &str) {
    conn.execute(
        "UPDATE hello_world_messages SET name = ?1, message = ?2 WHERE id = ?3",
        params![name, message, id],
    )
    .unwrap();
}

pub fn get_messages(conn: &Connection, id: &i8) -> HelloWorldResponse {
    let mut stmt = conn.prepare("SELECT id, name, message FROM hello_world_messages WHERE id = ?1").unwrap();
    let mut rows = stmt.query(params![id]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get(1).unwrap();
        let message: String = row.get(2).unwrap();
        return HelloWorldResponse {
            message: format!("Id: {}, Name: {}, Message: {}", id, name, message),
        };
    }
    HelloWorldResponse {
        message: "No message found".to_string(),
    }
}

pub fn delete_message(conn: &Connection, id: i8) -> String {
    let result = conn.execute("DELETE FROM hello_world_messages WHERE id = ?1", params![id]);
    match result {
        Ok(_) => format!("Message with id {} deleted successfully", id),
        Err(_) => format!("Failed to delete message with id {}", id),
    }
}
