mod create_tasks;
mod get_tasks;
mod update_tasks;

use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::routes::tasks::{
    create_tasks::create_task,
    get_tasks::{get_all_tasks, get_one_task},
    update_tasks::{atomic_update_task, partial_update_task},
};

pub fn create_task_routes() -> Router {
    Router::new()
        .route("/tasks", get(get_all_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_one_task))
        .route("/tasks/{id}", put(atomic_update_task))
        .route("/tasks/{id}", patch(partial_update_task))
}
