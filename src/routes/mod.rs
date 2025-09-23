mod hello_world;
mod mirror_body;
mod params;

use axum::{
    Router,
    routing::{get, post},
};
use hello_world::hello_world;

use crate::routes::{
    mirror_body::{
        mirror_body_json_handler, mirror_body_string_handler, mirror_custom_header_handler,
        mirror_user_agent_handler,
    },
    params::{path_params_handler, query_params_handler},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string_handler))
        .route("/mirror_body_json", post(mirror_body_json_handler))
        .route("/path_variables/{id}", get(path_params_handler))
        .route("/query_params", get(query_params_handler))
        .route("/mirror_user_agent", get(mirror_user_agent_handler))
        .route("/mirror_custom_header", get(mirror_custom_header_handler))
}
