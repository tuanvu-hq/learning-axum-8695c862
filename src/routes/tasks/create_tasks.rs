use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::Deserialize;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn create_task(
    Extension(db): Extension<DatabaseConnection>,
    Json(task): Json<RequestTask>,
) -> Result<StatusCode, StatusCode> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title),
        description: Set(task.description),
        ..Default::default()
    };

    new_task
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
