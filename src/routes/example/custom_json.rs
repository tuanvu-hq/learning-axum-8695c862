use axum::{
    Json, RequestExt,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn json_get_one_handler() -> Json<Data> {
    let data = Data {
        message: "JSON message".to_owned(),
        count: 18,
        username: "user-1".to_owned(),
    };

    Json(data)
}

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: Option<String>,
}

pub async fn json_validate_handler(Json(user): Json<RequestUser>) -> String {
    format!(
        "Hello {}, your password is: {}.",
        user.username,
        user.password.unwrap_or("hidden".to_owned())
    )
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestUserTwo {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

impl<S> FromRequest<S> for RequestUserTwo
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUserTwo>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}

pub async fn json_extractor_handler(user: RequestUserTwo) -> Json<RequestUserTwo> {
    Json(user)
}
