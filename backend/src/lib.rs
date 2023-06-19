pub mod database;
pub mod elsevier_data;
pub mod models;
pub mod springer_data;

use axum::{
    extract::{Query, State},
    http::StatusCode,
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
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn app(client: mongodb::Client) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("frontend/build"))
        .route("/articles", routing::get(get_articles))
        .route("/random-article", routing::get(get_random_article))
        .layer(CorsLayer::permissive())
        .with_state(client.clone())
}

#[derive(Debug, Deserialize)]
pub struct UrlArguments {
    pub _id: Option<bson::oid::ObjectId>,
    pub publication_date: Option<NaiveDate>,
    pub n_per_page: Option<i64>,
    pub query: Option<String>,
    pub source: Option<String>,
    pub descending: Option<bool>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug)]
pub struct FindArguments {
    pub _id: Option<bson::oid::ObjectId>,
    pub publication_date: Option<bson::datetime::DateTime>,
    pub n_per_page: i64,
    pub query: Option<String>,
    pub source: Option<String>,
    pub descending: bool,
    pub start_date: Option<bson::datetime::DateTime>,
    pub end_date: Option<bson::datetime::DateTime>,
}

fn naive_to_bson(date: NaiveDate) -> Result<bson::datetime::DateTime, models::ParseError> {
    bson::DateTime::builder()
        .year(date.year())
        .month(date.month() as u8)
        .day(date.day() as u8)
        .build()
        .ok()
        .ok_or(models::ParseError)
}

impl TryFrom<UrlArguments> for FindArguments {
    type Error = models::ParseError;

    fn try_from(arguments: UrlArguments) -> Result<Self, Self::Error> {
        let mut publication_date: Option<bson::datetime::DateTime> = None;
        let mut start_date: Option<bson::datetime::DateTime> = None;
        let mut end_date: Option<bson::datetime::DateTime> = None;

        if let Some(start_naive_date) = arguments.start_date {
            let start_bson_date = naive_to_bson(start_naive_date)?;
            if let Some(end_naive_date) = arguments.end_date {
                let end_bson_date = naive_to_bson(end_naive_date)?;
                start_date = Some(start_bson_date);
                end_date = Some(end_bson_date);
            }
        }

        if let Some(naive_date) = arguments.publication_date {
            let date_bson = naive_to_bson(naive_date)?;
            publication_date = Some(date_bson);
        }

        let mut n_per_page = 30;
        if let Some(n) = arguments.n_per_page {
            if n > 0 {
                n_per_page = n;
            } else {
                return Err(models::ParseError);
            }
        }

        let mut descending = true;
        if let Some(value) = arguments.descending {
            descending = value;
        }

        return Ok(Self {
            _id: arguments._id,
            publication_date,
            n_per_page,
            query: arguments.query,
            source: arguments.source,
            descending,
            start_date,
            end_date,
        });
    }
}

impl Default for UrlArguments {
    fn default() -> Self {
        Self {
            _id: None,
            publication_date: None,
            n_per_page: Some(30),
            query: None,
            source: None,
            descending: Some(true),
            start_date: None,
            end_date: None,
        }
    }
}

pub async fn get_articles(
    State(client): State<mongodb::Client>,
    arguments: Option<Query<UrlArguments>>,
) -> response::Response {
    let db = client.database("bioinf-news");
    let Query(url_arguments) = arguments.unwrap_or_default();
    if let Ok(arguments) = FindArguments::try_from(url_arguments) {
        match find_many_articles(&db, arguments).await {
            Ok(mut cursor) => {
                let mut articles: Vec<models::ArticleOutgoing> = Vec::new();
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(article_short) => {
                            articles.push(models::ArticleOutgoing::from(article_short));
                        }
                        Err(_) => {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    }
                }
                if articles.len() == 0 {
                    return StatusCode::NOT_FOUND.into_response();
                }
                return response::Json::from(articles).into_response();
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
    }
    StatusCode::BAD_REQUEST.into_response()
}

pub async fn get_random_article(State(client): State<mongodb::Client>) -> response::Response {
    let db = client.database("bioinf-news");
    match find_one_random_article(&db).await {
        Ok(mut cursor) => {
            if let Some(doc) = cursor.next().await {
                match doc {
                    Ok(doc) => {
                        let article: Result<models::Article, mongodb::bson::de::Error> =
                            bson::from_document(doc);
                        match article {
                            Ok(article) => {
                                let article = models::ArticleOutgoing::from(article);
                                return response::Json::from(article).into_response();
                            }
                            Err(_) => {
                                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                            }
                        }
                    }
                    Err(_) => {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
            }
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn find_many_articles(
    db: &mongodb::Database,
    arguments: FindArguments,
) -> mongodb::error::Result<mongodb::Cursor<models::Article>> {
    let collection = db.collection::<models::Article>("articles");

    // SORT ORDER
    let mut sort_order = 1;
    if arguments.descending {
        sort_order = -1;
    }

    let mut filter = doc! {};

    // SEARCH
    if let Some(query) = arguments.query {
        filter.insert("$text", doc! {"$search": query});
    }

    // SOURCE
    if let Some(source) = arguments.source {
        filter.insert("source", source);
    }

    // SEARCH BY DATE
    if let Some(start_date) = arguments.start_date {
        if let Some(end_date) = arguments.end_date {
            let conditions: Vec<bson::Document> = vec![
                doc! {"publication_date": {"$lte": end_date}},
                doc! {"publication_date": {"$gte": start_date}},
            ];
            filter.insert("$and", conditions);
        }
    }

    // PAGINATION
    let n_per_page = arguments.n_per_page;
    if let Some(id) = &arguments._id {
        if let Some(publication_date) = &arguments.publication_date {
            filter.insert("publication_date", doc! {"$lte": publication_date});
            filter.insert("_id", doc! {"$lt": id});
        }
    }
    let options = FindOptions::builder()
        .sort(doc!("publication_date": sort_order, "_id": -1))
        .limit(n_per_page)
        .build();

    collection.find(filter, options).await
}

pub async fn find_one_random_article(
    db: &mongodb::Database,
) -> mongodb::error::Result<mongodb::Cursor<bson::Document>> {
    let collection = db.collection::<models::Article>("articles");
    let pipeline = vec![doc! {"$sample": {"size": 1}}];
    collection.aggregate(pipeline, None).await
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
