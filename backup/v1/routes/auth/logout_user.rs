use axum::{Extension, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, IntoActiveModel};

use crate::database::users;

pub async fn logout(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);
    user.save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
