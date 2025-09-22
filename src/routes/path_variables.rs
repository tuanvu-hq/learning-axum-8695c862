use axum::extract::Path;

pub async fn path_variables_handler(Path(id): Path<i32>) -> String {
    id.to_string()
}
