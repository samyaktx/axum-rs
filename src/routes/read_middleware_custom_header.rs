use axum::Extension;

#[derive(Clone)]
pub struct CustomMsgHeader(pub String);

pub async fn read_middleware_custom_header(Extension(message): Extension<CustomMsgHeader>) -> String {
    message.0
}
