use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn always_error_handler() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}

pub async fn status_code_201_handler() -> Response {
    // requires 'IntoResponse' trait
    (StatusCode::CREATED, "This is a 201".to_owned()).into_response()
}
