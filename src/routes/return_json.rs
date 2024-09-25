use axum::Json;
use serde::Serialize;


#[derive(Clone, Serialize)]
pub struct UserData {
    username: String,
    password: String,
}

pub async fn return_json() -> Json<UserData> {
    let user = UserData {
        username: "John Doe".to_owned(),
        password: "123456".to_owned(),
    };

    Json(user)
}

