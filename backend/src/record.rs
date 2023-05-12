use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    publication_date: bson::datetime::DateTime,
    title: String,
    authors: Vec<String>,
    key_words: Vec<String>,
    url: String,
    article_abstract: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    creation_date: bson::datetime::DateTime,
    article: Article,
}

impl Article {
    fn from_springer() {}
}

impl Record {
    fn new() {}
}
