use chrono::{Datelike, NaiveDate};
use mongodb::bson::{self, doc};
use serde::Deserialize;
use super::models;

fn naive_to_bson(date: NaiveDate) -> Result<bson::datetime::DateTime, models::ParseError> {
    bson::DateTime::builder()
        .year(date.year())
        .month(date.month() as u8)
        .day(date.day() as u8)
        .build()
        .ok()
        .ok_or(models::ParseError)
}

#[derive(Debug, Deserialize)]
pub struct UrlPagination {
    pub _id: Option<bson::oid::ObjectId>,
    pub publication_date: Option<NaiveDate>,
    pub n_per_page: i64,
    pub descending: bool,
}

impl Default for UrlPagination {
    fn default() -> Self {
        Self {
            _id: None,
            publication_date: None,
            n_per_page: 30,
            descending: true,
        }
    }
}

#[derive(Debug)]
pub struct Pagination {
    pub _id: Option<bson::oid::ObjectId>,
    pub publication_date: Option<bson::datetime::DateTime>,
    pub n_per_page: i64,
    pub descending: bool,
}

impl TryFrom<UrlPagination> for Pagination {
    type Error = models::ParseError;

    fn try_from(pagination: UrlPagination) -> Result<Self, Self::Error> {
        let mut publication_date: Option<bson::datetime::DateTime> = None;

        if let Some(naive_date) = pagination.publication_date {
            let date_bson = naive_to_bson(naive_date)?;
            publication_date = Some(date_bson);
        }

        let n_per_page = pagination.n_per_page;
        if n_per_page < 0 {
            return Err(models::ParseError);
        }

        return Ok(Self {
            _id: pagination._id,
            publication_date,
            n_per_page,
            descending: pagination.descending,
        });
    }
}

#[derive(Debug, Deserialize)]
pub struct UrlArguments {
    pub query: Option<String>,
    pub source: Option<String>,
    pub key_words: Option<Vec<String>>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug)]
pub struct FindArguments {
    pub query: Option<String>,
    pub source: Option<String>,
    pub key_words: Option<Vec<String>>,
    pub start_date: Option<bson::datetime::DateTime>,
    pub end_date: Option<bson::datetime::DateTime>,
}

impl TryFrom<UrlArguments> for FindArguments {
    type Error = models::ParseError;

    fn try_from(arguments: UrlArguments) -> Result<Self, Self::Error> {
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

        return Ok(Self {
            query: arguments.query,
            source: arguments.source,
            key_words: arguments.key_words,
            start_date,
            end_date,
        });
    }
}

impl Default for UrlArguments {
    fn default() -> Self {
        Self {
            query: None,
            source: None,
            key_words: None,
            start_date: None,
            end_date: None,
        }
    }
}

