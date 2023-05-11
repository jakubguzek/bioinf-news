pub mod database;
pub mod springer_data;

use axum::response::{self, IntoResponse};
use mongodb::Client;

pub async fn update_database(client: &Client) {}

// Test returning response json from Springer API.
pub async fn springer() -> response::Response {
    match springer_data::load_data().await {
        Ok(json) => response::Json::from(json).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
