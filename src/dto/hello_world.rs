use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HelloWorldRequest {
    pub name: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct HelloWorldResponse {
    pub message: String,
}
