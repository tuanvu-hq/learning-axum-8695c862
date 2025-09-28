pub mod common;
#[allow(unused_imports)]
pub mod database;
pub mod middleware;
pub mod queries;
pub mod router;
pub mod routes;
pub mod utils;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::{common::app_state::AppState, router::create_routes};

pub async fn run(app_state: AppState) {
    let app = create_routes(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server"); // 'unwrap' with custom err message
}
