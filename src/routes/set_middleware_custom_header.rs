use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::read_middleware_custom_header::CustomMsgHeader;

// curl -H "message: Hello World" http://localhost:3000/read_middleware_custom_header

pub async fn set_middleware_custom_header(mut request: Request<Body>, next: Next) -> Result<Response, axum::http::StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let message = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();
    
    let extension = request.extensions_mut();
    extension.insert(CustomMsgHeader(message));

    Ok(next.run(request).await)
}
