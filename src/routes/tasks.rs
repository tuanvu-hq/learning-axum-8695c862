use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde::Deserialize;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

pub async fn create_task_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(task): Json<RequestTask>,
) {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title),
        description: Set(task.description),
        ..Default::default()
    };
    let result = new_task.save(&db).await.unwrap();

    dbg!(result);
}
