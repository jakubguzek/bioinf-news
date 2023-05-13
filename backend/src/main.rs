use axum::routing;
use dotenv::dotenv;

/// Just a quick example main with tokio and axum Router.
#[tokio::main]
async fn main() {
    dotenv().ok();
    // Get request on /springer endpoint.
    let app = axum::Router::new().route("/springer", routing::get(backend::springer));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
