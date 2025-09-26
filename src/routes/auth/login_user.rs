use axum::{Extension, Json, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::database::users;

#[allow(dead_code)]
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

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(user): Json<RequestLoginUser>,
) -> Result<Json<ResponseLoginUser>, StatusCode> {
    let login_user = users::Entity::find()
        .filter(users::Column::Username.eq(user.username))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    if !verify_password(user.password, &login_user.password)? {
        return Err(StatusCode::UNAUTHORIZED);
    }

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

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
