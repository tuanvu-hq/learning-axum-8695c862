mod create_tasks;
mod delete_tasks;
mod get_tasks;
mod update_tasks;

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::routes::tasks::{
    create_tasks::create_task,
    delete_tasks::delete_task,
    get_tasks::{get_task, get_tasks},
    update_tasks::{atomic_update_task, partial_update_task},
};

pub fn create_task_routes() -> Router {
    Router::new()
        .route("/tasks", get(get_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks/{id}", put(atomic_update_task))
        .route("/tasks/{id}", patch(partial_update_task))
        .route("/tasks/{id}", delete(delete_task))
}
