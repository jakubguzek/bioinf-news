pub mod database;
pub mod models;
pub mod springer_data;
pub mod elsevier_data;

use axum::{
    extract::{Path, Query, State},
    response::{self, IntoResponse},
    routing, Router,
};
use chrono::{Datelike, Local, Months, NaiveDate};
use futures_util::stream::StreamExt;
use mongodb::{
    self,
    bson::{self, doc},
    options::{FindOptions, InsertManyOptions},
};
use serde::Deserialize;

pub fn app(client: mongodb::Client) -> Router {
    Router::new()
        .route("/articles", routing::get(get_articles_endpoint))
        .route("/articles/:id", routing::get(get_article_endpoint))
        .with_state(client.clone())
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub _id: Option<bson::oid::ObjectId>,
    pub publication_date: Option<NaiveDate>,
    pub n_per_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationIngoing {
    pub _id: Option<bson::oid::ObjectId>,
    pub publication_date: Option<bson::datetime::DateTime>,
    pub n_per_page: i64,
}

impl From<Pagination> for PaginationIngoing {
    fn from(pagination: Pagination) -> Self {
        match pagination.publication_date {
            Some(publication_date) => {
                let publication_date = bson::DateTime::builder()
                    .year(publication_date.year())
                    .month(publication_date.month() as u8)
                    .day(publication_date.day() as u8)
                    .build()
                    .unwrap();
                Self {
                    _id: pagination._id.clone(),
                    publication_date: Some(publication_date),
                    n_per_page: pagination.n_per_page.clone(),
                }
            }
            None => Self {
                _id: pagination._id.clone(),
                publication_date: None,
                n_per_page: pagination.n_per_page.clone(),
            },
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            _id: None,
            publication_date: None,
            n_per_page: 30,
        }
    }
}

pub async fn get_articles_endpoint(
    State(client): State<mongodb::Client>,
    pagination: Option<Query<Pagination>>,
) -> response::Response {
    let db = client.database("bioinf-news");
    let Query(pagination) = pagination.unwrap_or_default();
    match get_articles(&db, PaginationIngoing::from(pagination)).await {
        Ok(mut cursor) => {
            let mut articles: Vec<models::ArticleShortOutgoing> = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(article_short) => {
                        articles.push(models::ArticleShortOutgoing::from(article_short));
                    }
                    Err(_) => {
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
            }
            if articles.len() == 0 {
                return axum::http::StatusCode::NOT_FOUND.into_response();
            }
            return response::Json::from(articles).into_response();
        }
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// doi nie może być w path
pub async fn get_article_endpoint(
    Path(id): Path<bson::oid::ObjectId>,
    State(client): State<mongodb::Client>,
) -> response::Response {
    let db = client.database("bioinf-news");
    match get_article(&db, &id).await {
        Ok(result) => match result {
            Some(article) => {
                response::Json::from(models::ArticleOutgoing::from(article)).into_response()
            }
            None => axum::http::StatusCode::NOT_FOUND.into_response(),
        },
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_articles(
    db: &mongodb::Database,
    pagination: PaginationIngoing,
) -> mongodb::error::Result<mongodb::Cursor<models::ArticleShort>> {
    let collection = db.collection::<models::ArticleShort>("articles");
    let n_per_page = pagination.n_per_page;
    let mut filter = doc!();
    if let Some(id) = &pagination._id {
        if let Some(publication_date) = &pagination.publication_date {
            filter = doc!(
                "publication_date": {"$lte": publication_date},
                "_id": {"$lt": id},
            );
        }
    }
    let options = FindOptions::builder()
        .sort(doc!("publication_date": -1, "_id": -1))
        .limit(n_per_page)
        .projection(doc!("_id": 1, "doi": 1, "source": 1, "publication_date": 1, "title": 1))
        .build();
    return collection.find(filter, options).await;
}

pub async fn get_article(
    db: &mongodb::Database,
    id: &bson::oid::ObjectId,
) -> mongodb::error::Result<Option<models::Article>> {
    let collection = db.collection::<models::Article>("articles");
    let filter = doc!("_id": id);
    return collection.find_one(filter, None).await;
}

pub async fn delete_old_articles(
    db: &mongodb::Database,
) -> mongodb::error::Result<mongodb::results::DeleteResult> {
    let collection = db.collection::<models::Article>("articles");
    let old_date = Local::now().checked_sub_months(Months::new(1)).unwrap();
    let query = doc!("publication_date": {"$lt": bson::DateTime::from_chrono(old_date)});
    collection.delete_many(query, None).await
}

pub async fn update_articles_springer(
    db: &mongodb::Database,
    mut amount: usize,
    step: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    delete_old_articles(&db).await?;
    let mut records_buffer: Vec<models::Article> = Vec::with_capacity(step);

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
            .ok_or(models::ParseError)?
            .as_array()
            .ok_or(models::ParseError)?;
        for record_value in records {
            if let Ok(record) = models::Article::from_springer(record_value) {
                records_buffer.push(record);
            }
        }
        let options = InsertManyOptions::builder().ordered(false).build();
        let collection = db.collection::<models::Article>("articles");
        match collection.insert_many(&records_buffer, options).await {
            Err(insert_error) => match *insert_error.kind {
                mongodb::error::ErrorKind::BulkWrite(ref e) => {
                    if e.write_concern_error.is_some() {
                        panic!("Unexpected error occured: {:?}", &insert_error);
                    }
                }
                _ => {
                    panic!("Unexpected error occured: {:?}", &insert_error);
                }
            },
            Ok(_) => (),
        };
        idx += step;
        records_buffer.clear();
    }
    Ok(())
}
