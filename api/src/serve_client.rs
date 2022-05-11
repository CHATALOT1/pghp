use axum::{
    http::{uri::Uri, StatusCode},
    response::IntoResponse,
    routing::{get, get_service, Router},
};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn main(dir: PathBuf) -> Router {
    Router::new()
        .route(
            "/",
            get_service(ServeDir::new(dir)).handle_error(unexpected_error_serving_client),
        )
        .fallback(get(not_found_client))
}

async fn unexpected_error_serving_client(_err: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unexpected error found!\nYou could be in for some special 'thank you for \
        finding a bug' headpats!\nContact the Administrator of this instance or create \
        an issue at https://github.com/CHATALOT1/pghp",
    )
}

// TODO: Proper 404 page. See #5
async fn not_found_client(uri: Uri) -> impl IntoResponse {
    format!(
        "The page you were looking for ('{:?}') couldn't be found.",
        uri
    )
}
