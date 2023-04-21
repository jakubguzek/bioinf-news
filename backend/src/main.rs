use axum::{
    response::{self, IntoResponse},
    routing,
};

mod springer_data;

/// Just a quick example main with tokio and axum Router.
#[tokio::main]
async fn main() {
    // Get request on /springer endpoint.
    let app = axum::Router::new().route("/springer", routing::get(springer));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Test returning response json from Springer API.
async fn springer() -> response::Response {
    match springer_data::load_data().await {
        Ok(json) => response::Json::from(json).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
