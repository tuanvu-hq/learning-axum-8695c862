use axum::{
    Extension, Router, middleware,
    routing::{get, post},
};

use crate::routes::example::{
    custom_json::{json_extractor_handler, json_get_one_handler, json_validate_handler},
    custom_middleware::{
        middleware_custom_header_extractor, middleware_custom_header_handler,
        middleware_message_handler,
    },
    hello_world::hello_world_handler,
    mirror_body::{
        mirror_body_json_handler, mirror_body_string_handler, mirror_custom_header_handler,
        mirror_user_agent_handler,
    },
    params::{path_params_handler, query_params_handler},
    status_codes::{always_error_handler, status_code_201_handler},
};

mod custom_json;
mod custom_middleware;
mod hello_world;
mod mirror_body;
mod params;
mod status_codes;

#[derive(Clone)]
pub struct SharedData {
    message: String,
}

pub fn create_example_routes() -> Router {
    let shared_data = SharedData {
        message: "Middleware shared data".to_owned(),
    };

    Router::new()
        .route("/", get(hello_world_handler))
        .route(
            "/middleware_custom_header",
            get(middleware_custom_header_handler),
        )
        .route_layer(middleware::from_fn(middleware_custom_header_extractor))
        .route("/mirror_body_string", post(mirror_body_string_handler))
        .route("/mirror_body_json", post(mirror_body_json_handler))
        .route("/mirror_user_agent", get(mirror_user_agent_handler))
        .route("/mirror_custom_header", get(mirror_custom_header_handler))
        .route("/middleware_message", get(middleware_message_handler))
        .layer(Extension(shared_data))
        .route("/path_variables/{id}", get(path_params_handler))
        .route("/query_params", get(query_params_handler))
        .route("/status_codes_always_error", get(always_error_handler))
        .route("/status_codes_return_201", post(status_code_201_handler))
        .route("/json_get_one", get(json_get_one_handler))
        .route("/json_validate", post(json_validate_handler))
        .route("/json_extractor", post(json_extractor_handler))
}
