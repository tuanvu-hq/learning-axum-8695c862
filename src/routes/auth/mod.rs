use axum::{Extension, Json, Router, http::StatusCode, routing::post};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::database::users;

pub fn create_auth_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[derive(Deserialize)]
pub struct RequestRegisterUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseRegisterUser {
    id: i32,
    username: String,
    token: String,
}

async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user): Json<RequestRegisterUser>,
) -> Result<Json<ResponseRegisterUser>, StatusCode> {
    let register_user = users::ActiveModel {
        username: Set(user.username),
        password: Set(user.password), // !!!DEV-PURPOSE!!! calm down. plain-text password
        token: Set(Some("a6sd54g65sag4".to_owned())), // dev - temporary
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseRegisterUser {
        id: register_user.id.unwrap(),             // unwrap ActiveValue<String>
        username: register_user.username.unwrap(), // unwrap ActiveValue<String>
        token: register_user.token.unwrap().unwrap(), // unwrap ActiveValue<Option<String>>
    }))
}

#[derive(Deserialize)]
pub struct RequestLoginUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseLoginUser {
    id: i32,
    username: String,
    token: String,
}

async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(user): Json<RequestLoginUser>,
) -> Result<Json<ResponseLoginUser>, StatusCode> {
    let login_user = users::Entity::find()
        .filter(users::Column::Username.eq(user.username))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;
    let new_token = "as54dad465aff".to_owned();
    let mut user = login_user.into_active_model();

    user.token = Set(Some(new_token));

    let login_user = user
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseLoginUser {
        id: login_user.id.unwrap(),
        username: login_user.username.unwrap(),
        token: login_user.token.unwrap().unwrap(),
    }))
}
