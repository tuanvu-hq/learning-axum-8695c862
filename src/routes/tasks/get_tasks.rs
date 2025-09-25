use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::tasks;

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
}

pub async fn get_task(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = tasks::Entity::find_by_id(id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&db)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ResponseTask {
        id: task.id,
        priority: task.priority,
        title: task.title,
        description: task.description,
        deleted_at: task.deleted_at,
    }))
}

#[derive(Deserialize)]
pub struct QueryParams {
    priority: Option<String>,
}

pub async fn get_tasks(
    Query(params): Query<QueryParams>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();

    match params.priority {
        Some(priority) if priority.is_empty() => {
            priority_filter = priority_filter.add(tasks::Column::Priority.is_null());
        }
        Some(priority) => {
            priority_filter = priority_filter.add(tasks::Column::Priority.eq(priority));
        }
        None => {}
    }

    let tasks = tasks::Entity::find()
        .filter(priority_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|item| ResponseTask {
            id: item.id,
            priority: item.priority,
            title: item.title,
            description: item.description,
            deleted_at: item.deleted_at,
        })
        .collect();

    Ok(Json(tasks))
}
