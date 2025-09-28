use std::time::Duration;

use dotenvy::dotenv;
use learning_axum_8695c862::{
    common::app_state::AppState, run, utils::token_wrapper::TokenWrapper,
};
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Missing environment variable DATABASE_URL")
        .to_owned();
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("Missing environment variable JWT_SECRET")
        .to_owned();
    let mut db_options = ConnectOptions::new(database_url);
    db_options
        .min_connections(1)
        .max_connections(5)
        .connect_timeout(Duration::from_secs(8));
    let db = match Database::connect(db_options).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await;
}
