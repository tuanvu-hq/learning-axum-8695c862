use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    id: i32,
    message: String,
}

pub async fn query_params_handler(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
