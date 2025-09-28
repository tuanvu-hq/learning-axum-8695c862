pub mod create_tasks;
pub mod delete_tasks;
pub mod get_tasks;
pub mod task_extractors;
pub mod update_tasks;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    common::app_state::AppState,
    routes::tasks::{
        create_tasks::create_task,
        delete_tasks::soft_delete_task,
        get_tasks::{get_all_tasks, get_one_task},
        update_tasks::partial_update_task,
    },
};

pub fn create_task_routes() -> Router<AppState> {
    Router::new()
        .route("/tasks", get(get_all_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_one_task))
        .route("/tasks/{id}", patch(partial_update_task))
        .route("/tasks/{id}", delete(soft_delete_task))
}

#[derive(Deserialize, Serialize)]
pub struct RequestTask {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
