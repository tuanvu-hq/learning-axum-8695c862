use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    DatabaseConnection, EntityTrait, IntoActiveModel,
    prelude::DateTimeWithTimeZone,
};
use serde::Deserialize;

use crate::database::tasks;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct RequestAtomicTask {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

// # The order of extractors
// - 'Method' and 'HeaderMap' don't consume the request body so they can put anywhere in the argument list (but before 'body').
// - 'State' is also an extractor so it needs to be before 'body'.
// - 'String' or 'JSON' consume the body, and thus must be the last extractor.
// Note: Since parsing JSON requires consuming the request body, the Json extractor must be last if there are multiple extractors in a handler.
pub async fn atomic_update_task(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(task): Json<RequestAtomicTask>,
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Unchanged(id),
        priority: Set(task.priority),
        title: Set(task.title),
        completed_at: Set(task.completed_at),
        description: Set(task.description),
        deleted_at: Set(task.deleted_at),
        user_id: Set(task.user_id),
        is_default: Set(task.is_default),
    };

    update_task
        .update(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct RequestPartialTask {
    pub id: Option<i32>,
    // - 'default': important for deserialization
    // - 'skip_serializing_if': important for serialization. e.g. skip_serializing_if = "Option::is_none"
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub user_id: Option<Option<i32>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub is_default: Option<Option<bool>>,
}

pub async fn partial_update_task(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
    Json(task): Json<RequestPartialTask>,
) -> Result<(), StatusCode> {
    let mut update_task = tasks::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into_active_model();

    if let Some(priority) = task.priority {
        update_task.priority = Set(priority);
    }
    if let Some(title) = task.title {
        update_task.title = Set(title);
    }
    if let Some(completed_at) = task.completed_at {
        update_task.completed_at = Set(completed_at);
    }
    if let Some(description) = task.description {
        update_task.description = Set(description);
    }
    if let Some(deleted_at) = task.deleted_at {
        update_task.deleted_at = Set(deleted_at);
    }
    if let Some(user_id) = task.user_id {
        update_task.user_id = Set(user_id);
    }
    if let Some(is_default) = task.is_default {
        update_task.is_default = Set(is_default);
    }

    update_task
        .update(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
