use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use learning_axum_8695c862::run;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_uri = dotenv!("DATABASE_URL");

    run(db_uri).await;
}
