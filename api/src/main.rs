mod settings;

use axum::{routing::get, Router};
use std::net::SocketAddr;

use settings::get_instance_settings;

#[tokio::main]
async fn main() {
    let cfg = get_instance_settings();
    let app = Router::new().route("/*_", get(|| async { "More coming soon!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
