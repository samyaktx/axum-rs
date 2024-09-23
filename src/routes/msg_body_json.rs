use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonMsg {
    message: String,
}

#[derive(Serialize)]
pub struct JsonMsgResponse {
    message: String,
    message_from_server: String,
}

pub async fn msg_body_json(Json(body): Json<JsonMsg>) -> Json<JsonMsgResponse> {
    Json(JsonMsgResponse { 
        message: body.message, 
        message_from_server: "Hello from Mahalakshmi Universe".to_owned(), 
    })
}