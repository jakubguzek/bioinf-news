use axum::{routing, response};

/// Jsut a quick example main with tokio and axum Router.
#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", routing::get(root));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Bare-bones test root
async fn root() -> response::Html<&'static str>{
    response::Html("<h1>Hello World!</h1>")
}
