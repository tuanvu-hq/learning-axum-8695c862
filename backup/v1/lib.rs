#[allow(unused_imports)]
mod database;
mod routes;
mod utils;

use sea_orm::{ConnectOptions, Database};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;

use crate::routes::create_routes;

pub async fn run(db_uri: &str) {
    let mut db_options = ConnectOptions::new(db_uri);
    db_options
        .min_connections(1)
        .max_connections(5)
        .connect_timeout(Duration::from_secs(8));

    let db = Database::connect(db_options).await.unwrap();
    let app = create_routes(db);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server"); // 'unwrap' with custom err message
}
