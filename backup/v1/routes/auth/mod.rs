mod guard;
mod login_user;
mod logout_user;
mod register_user;

use axum::{Router, middleware, routing::post};

use crate::routes::auth::{login_user::login, logout_user::logout, register_user::register};

pub use guard::guard_user;

pub fn create_auth_routes() -> Router {
    Router::new()
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn(guard_user))
        .route("/register", post(register))
        .route("/login", post(login))
}
