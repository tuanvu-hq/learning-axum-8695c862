mod create_tasks;
mod get_tasks;

use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::tasks::{
    create_tasks::create_task_handler,
    get_tasks::{get_all_tasks_handler, get_one_task_handler},
};

pub fn create_task_routes() -> Router {
    Router::new()
        .route("/tasks", get(get_all_tasks_handler))
        .route("/tasks", post(create_task_handler))
        .route("/tasks/{id}", get(get_one_task_handler))
}
