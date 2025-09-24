use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::tasks;

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn get_one_task_handler(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = tasks::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ResponseTask {
        id: task.id,
        priority: task.priority,
        title: task.title,
        description: task.description,
    }))
}

pub async fn get_all_tasks_handler(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let tasks = tasks::Entity::find()
        .all(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|t| ResponseTask {
            id: t.id,
            priority: t.priority,
            title: t.title,
            description: t.description,
        })
        .collect();

    Ok(Json(tasks))
}
