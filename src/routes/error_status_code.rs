use axum::http::StatusCode;

pub async fn error_status_code() -> Result<(), StatusCode> {    
    Err(StatusCode::NOT_FOUND)
}