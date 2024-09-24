use axum::http::HeaderMap;

pub async fn header_custom_keyvalue(headers: HeaderMap) -> String {
    let header_value = headers.get("x-message").unwrap();
    let message = header_value.to_str().unwrap().to_owned();
    message
}