use axum::{routing::{get, post}, Router};

mod hello_world;
mod text_body_string;

use hello_world::hello_world;
use text_body_string::text_body_string;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
        .route("/text_body_string", post(text_body_string))  // shows http `body` in string text
}