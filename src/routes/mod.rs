use axum::{routing::{get, post}, Router};

mod hello_world;
mod text_body_string;
mod msg_body_json;

use hello_world::hello_world;
use msg_body_json::msg_body_json;
use text_body_string::text_body_string;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
        .route("/text_body_string", post(text_body_string))  // curl -v POST http://localhost:3000/text_body_string -d "Mahalakshmi I need help"
        .route("/msg_body_json", post(msg_body_json))  // curl -X POST http://localhost:3000/msg_body_json -H "Content-Type: application/json" -d '{"message": "I workship Mahalakshmi"}'

}