mod serve_client;
mod settings;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cfg = settings::get_instance_settings();

    let app = Router::new()
        .fallback(serve_client::main(cfg.client_dir));

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
