use crate::{
    common::app_error::AppError,
    database::users::{self, Entity as Users, Model as UserModel},
};
use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel,
};

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        match error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            true => AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            ),
            false => {
                eprintln!("Error creating user: {:?}", error_message);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again",
                )
            }
        }
    })?;

    convert_active_to_model(user)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UserModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by username: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?
        .ok_or_else(|| {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "incorrect username and/or password",
            )
        })
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);

        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
