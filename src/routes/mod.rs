mod custom_middleware;
mod hello_world;
mod mirror_body;
mod params;

use crate::routes::{
    custom_middleware::{
        middleware_custom_header_extractor, middleware_custom_header_handler,
        middleware_message_handler,
    },
    mirror_body::{
        mirror_body_json_handler, mirror_body_string_handler, mirror_custom_header_handler,
        mirror_user_agent_handler,
    },
    params::{path_params_handler, query_params_handler},
};

use axum::{
    Extension, Router,
    http::Method,
    middleware,
    routing::{get, post},
};
use hello_world::hello_world;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Middleware shared data".to_owned(),
    };

    Router::new()
        .route(
            "/middleware_custom_header",
            get(middleware_custom_header_handler),
        )
        .route_layer(middleware::from_fn(middleware_custom_header_extractor))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string_handler))
        .route("/mirror_body_json", post(mirror_body_json_handler))
        .route("/path_variables/{id}", get(path_params_handler))
        .route("/query_params", get(query_params_handler))
        .route("/mirror_user_agent", get(mirror_user_agent_handler))
        .route("/mirror_custom_header", get(mirror_custom_header_handler))
        .route("/middleware_message", get(middleware_message_handler))
        .layer(Extension(shared_data))
        .layer(cors)
}
