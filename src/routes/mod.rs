use axum::{routing::{get, post}, Router};

mod hello_world;
mod text_body_string;
mod msg_body_json;
mod path_variables;
mod query_params;
mod header_user_agent;

use hello_world::hello_world;
use msg_body_json::msg_body_json;
use text_body_string::text_body_string;
use path_variables::{hardcoded_path, path_variables};
use query_params::query_params;
use header_user_agent::header_user_agent;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
        .route("/text_body_string", post(text_body_string))  // curl -v POST http://localhost:3000/text_body_string -d "Mahalakshmi I need help"
        .route("/msg_body_json", post(msg_body_json))  // curl -X POST http://localhost:3000/msg_body_json -H "Content-Type: application/json" -d '{"message": "I workship Mahalakshmi"}'
        .route("/path_variables/:id", post(path_variables))  // curl -X POST http://localhost:3000/path_variables/117
        .route("/path_variables/17", post(hardcoded_path))  // curl -X POST http://localhost:3000/path_variables/17
        .route("/query_params", post(query_params))  // curl -X POST "http://localhost:3000/query_params?message=Hello%20world&id=117"
        .route("/header_user_agent", get(header_user_agent))  // curl -H "User-Agent: samyakt" -H "Content-Type: application/json" http://localhost:3000/header_user_agent
}