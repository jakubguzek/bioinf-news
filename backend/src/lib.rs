pub mod database;
pub mod record;
pub mod springer_data;

use axum::response::{self, IntoResponse};

// Test returning response json from Springer API.
pub async fn springer() -> response::Response {
    match springer_data::load_data().await {
        Ok(json) => response::Json::from(json).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn test_insert() {
    let client = database::connect_mongo_db().await.unwrap();
    let db = client.database("bioinf-news");

    // Get a handle to a collection in the database.
    let collection = db.collection::<record::Record>("records");
    let record = record::Record::from_springer(
        springer_data::load_data()
            .await
            .unwrap()
            .get("records")
            .expect("should have records field")
            .get(0)
            .expect("should have 0'th object"),
    )
    .unwrap();
    collection.insert_one(record, None).await.unwrap();
}
