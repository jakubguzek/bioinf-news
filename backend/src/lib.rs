pub mod database;
pub mod record;
pub mod springer_data;

use axum::response::{self, IntoResponse};

// Test returning response json from Springer API.
pub async fn springer() -> response::Response {
    match springer_data::load_data().await {
        Ok(json) => {
            for i in 0..100 {
                dbg!(i);
                dbg!(record::Record::new(
                   json.get("records").unwrap().get(i).unwrap()
                ));
            }
            response::Json::from(json).into_response()
        }
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
