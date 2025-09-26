use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::database::users;

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

pub async fn register(
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
