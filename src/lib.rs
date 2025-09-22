mod routes;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::routes::create_routes;

pub async fn run() {
    let app = create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server"); // 'unwrap' with custom err message
}
