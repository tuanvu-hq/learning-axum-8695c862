use crate::{
    common::app_error::AppError,
    database::users::{self, Entity as Users},
    utils::{jwt::validate_token, token_wrapper::TokenWrapper},
};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn require_authentication(
    headers: HeaderMap,
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let header_token = headers
        .get("x-auth-token")
        .ok_or(AppError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ))?
        .to_str()
        .map_err(|error| {
            eprintln!("Error extracting token from headers: {:?}", error);

            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?;

    validate_token(&token_secret.0, header_token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(header_token.to_owned())))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by token: {:?}", error);

            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    request.extensions_mut().insert(user.ok_or(AppError::new(
        StatusCode::UNAUTHORIZED,
        "You are not authorized for this",
    ))?);

    Ok(next.run(request).await)
}
