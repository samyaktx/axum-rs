use axum::{routing::{get, post}, Router};

mod hello_world;
mod text_body_string;
mod msg_body_json;
mod path_variables;

use hello_world::hello_world;
use msg_body_json::msg_body_json;
use text_body_string::text_body_string;
use path_variables::{hardcoded_path, path_variables};

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
        .route("/text_body_string", post(text_body_string))  // curl -v POST http://localhost:3000/text_body_string -d "Mahalakshmi I need help"
        .route("/msg_body_json", post(msg_body_json))  // curl -X POST http://localhost:3000/msg_body_json -H "Content-Type: application/json" -d '{"message": "I workship Mahalakshmi"}'
        .route("/path_variables/:id", post(path_variables))  // curl -X POST http://localhost:3000/path_variables/117
        .route("/path_variables/17", post(hardcoded_path))  // curl -X POST http://localhost:3000/path_variables/17

}