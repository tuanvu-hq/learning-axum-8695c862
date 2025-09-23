use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json_handler() -> Json<Data> {
    let data = Data {
        message: "JSON message".to_owned(),
        count: 18,
        username: "user-1".to_owned(),
    };

    Json(data)
}
