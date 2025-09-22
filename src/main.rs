use std::net::SocketAddr;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server"); // 'unwrap' with custom err message
}

async fn hello_world() -> String {
    "Hello World".to_owned() // equivalent to 'to_string'
}
