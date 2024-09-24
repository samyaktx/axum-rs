use axum_extra::{headers::UserAgent, TypedHeader};

pub async fn header_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}