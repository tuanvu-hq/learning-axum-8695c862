use axum::{Extension, Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{
    common::app_error::AppError,
    database::users,
    queries::task_queries,
    routes::tasks::{ResponseDataTask, ResponseTask, task_extractors::ValidateCreateTask},
};

#[axum::debug_handler]
pub async fn create_task(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;
    let response = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask { data: response }),
    ))
}
