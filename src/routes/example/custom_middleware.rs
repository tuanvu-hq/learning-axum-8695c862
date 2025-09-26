use axum::{
    Extension, body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response,
};

use super::SharedData;

pub async fn middleware_message_handler(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn middleware_custom_header_handler(
    Extension(message): Extension<HeaderMessage>,
) -> String {
    message.0
}

pub async fn middleware_custom_header_extractor(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers(); // &HeaderMap
    let message = headers.get("message"); // Option<&HeaderValue>
    let message = message.ok_or(StatusCode::BAD_REQUEST); // Result<&HeaderValue, StatusCode>
    let message = message?; // &HeaderValue
    let message = message.to_str().map_err(|_err| StatusCode::BAD_REQUEST); // Result<&str, StatusCode>
    let message = message?.to_owned(); // String
    let extensions = req.extensions_mut();

    extensions.insert(HeaderMessage(message));

    Ok(next.run(req).await)
}
