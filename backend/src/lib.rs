pub mod arguments;
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
use chrono::{Local, Months};
use futures_util::stream::StreamExt;
use mongodb::{
    self,
    bson::{self, doc},
    options::{FindOptions, InsertManyOptions},
};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub fn app(client: mongodb::Client) -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("frontend/build"))
        .route("/articles", routing::get(get_articles))
        .route("/keywords", routing::get(get_many_key_words))
        .route("/random-article", routing::get(get_random_article))
        .layer(CorsLayer::permissive())
        .with_state(client.clone())
}

pub async fn get_articles(
    State(client): State<mongodb::Client>,
    url_pagination: Option<Query<arguments::UrlPagination>>,
    url_arguments: Option<Query<arguments::UrlArguments>>,
) -> response::Response {
    let db = client.database("bioinf-news");
    let Query(pagination) = url_pagination.unwrap_or_default();
    let Query(arguments) = url_arguments.unwrap_or_default();
    if let Ok(arguments) = arguments::FindArguments::try_from(arguments) {
        if let Ok(pagination) = arguments::Pagination::try_from(pagination) {
            match find_many_articles(&db, &pagination, &arguments).await {
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
    }
    StatusCode::BAD_REQUEST.into_response()
}

pub async fn get_random_article(State(client): State<mongodb::Client>) -> response::Response {
    let db = client.database("bioinf-news");
    match find_one_random_article(&db).await {
        Ok(mut cursor) => {
            if let Some(result) = cursor.next().await {
                match result {
                    Ok(doc) => {
                        let result: Result<models::Article, mongodb::bson::de::Error> =
                            bson::from_document(doc);
                        match result {
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

pub async fn get_many_key_words(State(client): State<mongodb::Client>) -> response::Response {
    let db = client.database("bioinf-news");
    match find_many_key_words(&db).await {
        Ok(mut cursor) => {
            if let Some(result) = cursor.next().await {
                match result {
                    Ok(doc) => {
                        let result = doc.get_array("key_words");
                        match result {
                            Ok(key_words) => {
                                return response::Json::from(key_words).into_response();
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
    pagination: &arguments::Pagination,
    arguments: &arguments::FindArguments,
) -> mongodb::error::Result<mongodb::Cursor<models::Article>> {
    let collection = db.collection::<models::Article>("articles");

    let mut filter = doc! {};

    // PAGINATION AND SORT ORDER
    let n_per_page = pagination.n_per_page;
    if let Some(id) = &pagination._id {
        if let Some(publication_date) = &pagination.publication_date {
            filter.insert("publication_date", doc! {"$lte": publication_date});
            filter.insert("_id", doc! {"$lt": id});
        }
    }
    let mut sort_order = 1;
    if pagination.descending {
        sort_order = -1;
    }
    let options = FindOptions::builder()
        .sort(doc!("publication_date": sort_order, "_id": -1))
        .limit(n_per_page)
        .build();

    // SEARCH
    if let Some(query) = &arguments.query {
        filter.insert("$text", doc! {"$search": query});
    }

    // SOURCE
    if let Some(source) = &arguments.source {
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

    // KEYWORDS
    if let Some(key_words) = &arguments.key_words {
        filter.insert("key_words", doc! {"$all": key_words});
    }

    collection.find(dbg!(filter), options).await
}

pub async fn find_many_key_words(
    db: &mongodb::Database,
) -> mongodb::error::Result<mongodb::Cursor<bson::Document>> {
    let collection = db.collection::<models::Article>("articles");
    let group_stage = doc! {"$group": {
        "_id": 0,
        "key_words": {"$addToSet": "$key_words"}
    }};
    let project_stage = doc! {"$project": {
        "_id": 0,
        "key_words": {
            "$reduce": {
                "input": "$key_words",
                "initialValue": [],
                "in": {
                    "$setDifference": [
                        {"$concatArrays": ["$$this", "$$value"]},
                        []
                    ]
                }
            }
        }
    }};
    let pipeline = vec![group_stage, project_stage];
    collection.aggregate(pipeline, None).await
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
