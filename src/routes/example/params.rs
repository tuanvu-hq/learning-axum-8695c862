use axum::{
    Json,
    extract::{Path, Query},
};
use serde::{Deserialize, Serialize};

pub async fn path_params_handler(Path(id): Path<i32>) -> String {
    id.to_string()
}

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    id: i32,
    message: String,
}

pub async fn query_params_handler(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
