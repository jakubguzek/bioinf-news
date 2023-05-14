pub mod database;
pub mod record;
pub mod springer_data;

use axum::response::{self, IntoResponse};
use chrono::{Local, Months};

// Test returning response json from Springer API.
pub async fn springer() -> response::Response {
    match springer_data::load_data().await {
        Ok(json) => response::Json::from(json).into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_springer(
    mongo_db_client: mongodb::Client,
    mut amount: usize,
    step: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = mongo_db_client.database("bioinf-news");
    let collection = db.collection::<record::Record>("springer_records");

    let reqwest_client = reqwest::Client::new();

    let till_date = Local::now().date_naive();
    let from_date = till_date
        .clone()
        .checked_sub_months(Months::new(1))
        .unwrap();

    let get_url = |idx| {
        springer_data::springer_articles_url(
            "Bioinformatics",
            "Journal",
            &from_date,
            &till_date,
            idx,
            step,
        )
    };

    let get_total_records = |res: &serde_json::Value| -> Option<usize> {
        res.get("result")?
            .as_array()?
            .get(0)?
            .get("total")?
            .as_str()?
            .parse::<usize>()
            .ok()
    };

    let mut idx = 1;
    while idx < amount {
        let res = springer_data::springer_json_response(&reqwest_client, get_url(idx)).await?;
        if idx == 1 {
            if let Some(total_records) = get_total_records(&res) {
                if total_records < amount {
                    amount = total_records;
                }
            }
        }
        let records = res
            .get("records")
            .ok_or(record::ParseError)?
            .as_array()
            .ok_or(record::ParseError)?;
        for record_value in records {
            if let Ok(record) = record::Record::from_springer(record_value) {
                collection.insert_one(record, None).await?;
            }
        }
        idx += step;
    }

    Ok(())
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
