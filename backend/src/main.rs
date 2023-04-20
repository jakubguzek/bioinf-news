use axum::{routing::get, Router};
use std::net::SocketAddr;

/// Jsut a quick example main with tokio and axum Router.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Bare-bones test root
async fn root() -> &'static str {
    "Hello World!"
}
