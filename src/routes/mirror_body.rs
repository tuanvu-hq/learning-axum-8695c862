use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn mirror_body_string_handler(body: String) -> String {
    body
}

#[derive(Deserialize, Serialize)]
pub struct MirrorJsonBody {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
}

pub async fn mirror_body_json_handler(
    Json(body): Json<MirrorJsonBody>,
) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse {
        message: body.message,
    })
}
