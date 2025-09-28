use axum::{Extension, extract::State, http::StatusCode};
use sea_orm::{ActiveValue::Set, DatabaseConnection, IntoActiveModel};

use crate::{
    common::app_error::AppError, database::users, queries::user_queries::save_active_user,
};

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    save_active_user(&db, user).await?;

    Ok(StatusCode::OK)
}
