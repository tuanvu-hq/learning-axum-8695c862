use axum::{Extension, Json, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::Deserialize;

use crate::database::{tasks, users};

#[derive(Deserialize)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn create_task(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(db): Extension<DatabaseConnection>,
    Json(task): Json<RequestTask>,
) -> Result<StatusCode, StatusCode> {
    let token = authorization.token();
    let user = users::Entity::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    new_task
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
