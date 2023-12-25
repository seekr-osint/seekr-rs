use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<h1>404</h1><p>Not Found</p>"))
}
