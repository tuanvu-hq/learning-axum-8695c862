use axum::{Router, http::Method, middleware};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    common::app_state::AppState,
    middleware::require_authentication::require_authentication,
    routes::{
        auth::{create_auth_routes, create_non_auth_routes},
        tasks::create_task_routes,
    },
};

pub fn create_routes(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .nest("/api", create_task_routes())
        .merge(create_auth_routes())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .merge(create_non_auth_routes())
        .layer(cors)
        .with_state(app_state)
}
