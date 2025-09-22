mod hello_world;
mod mirror_body;
mod path_variables;

use axum::{
    Router,
    routing::{get, post},
};
use hello_world::hello_world;

use crate::routes::{
    mirror_body::{mirror_body_json_handler, mirror_body_string_handler},
    path_variables::path_variables_handler,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string_handler))
        .route("/mirror_body_json", post(mirror_body_json_handler))
        .route("/path_variables/{id}", get(path_variables_handler))
}
