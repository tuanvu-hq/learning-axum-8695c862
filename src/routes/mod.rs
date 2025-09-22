mod hello_world;
mod mirror_body;

use axum::{
    Router,
    routing::{get, post},
};
use hello_world::hello_world;
use mirror_body::{mirror_body_json, mirror_body_string};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}
