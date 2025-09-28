pub mod login_user;
pub mod logout_user;
pub mod register_user;

use axum::{Router, routing::post};
use serde::{Deserialize, Serialize};

use crate::{
    common::app_state::AppState,
    routes::auth::{login_user::login, logout_user::logout, register_user::register},
};

#[derive(Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}

pub fn create_auth_routes() -> Router<AppState> {
    Router::new().route("/logout", post(logout))
}

pub fn create_non_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
