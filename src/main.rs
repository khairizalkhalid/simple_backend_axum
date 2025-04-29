use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello-world", get(hello_world));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Start listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<&'static str> {
    Html("<div>Hello World</div>")
}
