use dotenv::dotenv;
use tokio;

/// Just a quick example main with tokio and axum Router.
#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = backend::database::connect_mongo_db().await.unwrap();
    backend::database::create_indexes(&client).await;
    let db = client.database("bioinf-news");
    backend::update_articles_springer(&db, 1000, 100)
        .await
        .unwrap();

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-> Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(backend::app(client).into_make_service())
        .await
        .unwrap();
}
