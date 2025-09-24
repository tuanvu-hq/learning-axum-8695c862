mod example;
mod tasks;

use axum::{Extension, Router, http::Method, routing::post};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{example::create_example_routes, tasks::create_task_handler};

pub fn create_routes(db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .nest("/api", create_task_routes())
        .layer(Extension(db))
        .nest("/example", create_example_routes())
        .layer(cors)
}

fn create_task_routes() -> Router {
    Router::new().route("/tasks", post(create_task_handler))
}
