use axum::{routing::get, Router};

pub fn main() -> Router {
    Router::new().route("/", get(temp_api_placeholder))
}

async fn temp_api_placeholder() -> &'static str {
    "API coming soon! :D"
}
