use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// curl -I http://localhost:3000/return_201_status_code
pub async fn return_201_status_code() -> Response {
    (StatusCode::CREATED, "").into_response()
}