use axum::{Json, http::HeaderMap};
use axum_extra::{TypedHeader, headers::UserAgent};
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

pub async fn mirror_user_agent_handler(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

pub async fn mirror_custom_header_handler(headers: HeaderMap) -> String {
    let message = headers.get("x-message").unwrap(); // &HeaderValue
    let message = message.to_str(); // Result<&str, ToStrError>
    let message = message.unwrap().to_string(); // String

    message
}
