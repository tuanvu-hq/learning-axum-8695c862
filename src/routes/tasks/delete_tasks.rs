#![allow(unused_imports)]

use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde::Deserialize;

use crate::database::tasks;

// # without soft delete
// pub async fn delete_task(
//     Path(id): Path<i32>,
//     Extension(db): Extension<DatabaseConnection>,
// ) -> Result<StatusCode, StatusCode> {
//     // # Option 1
//     // let task = tasks::Entity::find_by_id(id)
//     //     .one(&db)
//     //     .await
//     //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
//     //     .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
//     //     .into_active_model();

//     // tasks::Entity::delete(task)
//     //     .exec(&db)
//     //     .await
//     //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     // # Option 2
//     // tasks::Entity::delete_many()
//     //     .filter(tasks::Column::Id.eq(id))
//     //     .exec(&db)
//     //     .await
//     //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     // # Option 3
//     tasks::Entity::delete_by_id(id)
//         .exec(&db)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

//     Ok(StatusCode::NO_CONTENT)
// }

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

// # with soft delete
pub async fn delete_task(
    Path(id): Path<i32>,
    Query(params): Query<QueryParams>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<StatusCode, StatusCode> {
    if params.soft {
        let mut delete_task = tasks::Entity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            .into_active_model();
        let now = chrono::Utc::now();

        delete_task.deleted_at = Set(Some(now.into()));
        delete_task
            .update(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        tasks::Entity::delete_by_id(id)
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}
