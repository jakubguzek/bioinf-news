use chrono::{Datelike, NaiveDate};
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub _id: bson::oid::ObjectId,
    pub doi: String,
    pub source: String,
    pub publication_date: bson::datetime::DateTime,
    pub title: String,
    pub authors: Vec<String>,
    pub key_words: Vec<String>,
    pub urls: Vec<String>,
    pub article_abstract: String,
}

impl Article {
    pub fn from_springer(springer_record: &serde_json::Value) -> Result<Self, ParseError> {
        // mandatory
        let doi = springer_record
            .get("identifier")
            .ok_or(ParseError)?
            .as_str()
            .ok_or(ParseError)?
            .to_string();
        let publication_date = springer_record
            .get("onlineDate")
            .ok_or(ParseError)?
            .as_str()
            .ok_or(ParseError)?
            .to_string();
        let publication_date = NaiveDate::parse_from_str(&publication_date, "%Y-%m-%d")
            .ok()
            .ok_or(ParseError)?;
        let publication_date = bson::DateTime::builder()
            .year(publication_date.year())
            .month(publication_date.month() as u8)
            .day(publication_date.day() as u8)
            .build()
            .ok()
            .ok_or(ParseError)?;
        let title = springer_record
            .get("title")
            .ok_or(ParseError)?
            .as_str()
            .ok_or(ParseError)?
            .to_string();
        let urls = springer_record
            .get("url")
            .ok_or(ParseError)?
            .as_array()
            .ok_or(ParseError)?
            .iter()
            .filter_map(|x| x.get("value"))
            .filter_map(|x| x.as_str())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if urls.is_empty() {
            return Err(ParseError);
        }
        // optional
        let mut authors: Vec<String> = Vec::new();
        if let Some(authors_obj) = springer_record.get("creators") {
            if let Some(authors_array) = authors_obj.as_array() {
                authors = authors_array
                    .iter()
                    .filter_map(|x| x.get("creator"))
                    .filter_map(|x| x.as_str())
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
            }
        }
        let mut key_words: Vec<String> = Vec::new();
        if let Some(key_words_obj) = springer_record.get("subjects") {
            if let Some(key_words_array) = key_words_obj.as_array() {
                key_words = key_words_array
                    .iter()
                    .filter_map(|x| x.as_str())
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
            }
        }
        let mut article_abstract = String::new();
        if let Some(article_abstract_obj) = springer_record.get("abstract") {
            if let Some(article_abstract_str) = article_abstract_obj.as_str() {
                article_abstract = article_abstract_str.to_string();
            }
        }
        let source = String::from("springer");
        let _id = bson::oid::ObjectId::new();
        Ok(Self {
            _id,
            doi,
            source,
            publication_date,
            title,
            authors,
            key_words,
            urls,
            article_abstract,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleOutgoing {
    pub _id: bson::oid::ObjectId,
    pub doi: String,
    pub source: String,
    pub publication_date: chrono::NaiveDate,
    pub title: String,
    pub authors: Vec<String>,
    pub key_words: Vec<String>,
    pub urls: Vec<String>,
    pub article_abstract: String,
}

impl From<Article> for ArticleOutgoing {
    fn from(article: Article) -> Self {
        let publication_date = article.publication_date.to_chrono();
        let publication_date = chrono::NaiveDate::from_ymd_opt(
            publication_date.year(),
            publication_date.month(),
            publication_date.day(),
        )
        .unwrap();
        Self {
            _id: article._id.clone(),
            doi: article.doi.clone(),
            source: article.source.clone(),
            publication_date,
            title: article.title.clone(),
            authors: article.authors.clone(),
            key_words: article.key_words.clone(),
            urls: article.urls.clone(),
            article_abstract: article.article_abstract.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleShort {
    _id: bson::oid::ObjectId,
    doi: String,
    source: String,
    publication_date: bson::datetime::DateTime,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleShortOutgoing {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    _id: bson::oid::ObjectId,
    doi: String,
    source: String,
    publication_date: chrono::NaiveDate,
    title: String,
}

impl From<ArticleShort> for ArticleShortOutgoing {
    fn from(article_short: ArticleShort) -> Self {
        let publication_date = article_short.publication_date.to_chrono();
        let publication_date = chrono::NaiveDate::from_ymd_opt(
            publication_date.year(),
            publication_date.month(),
            publication_date.day(),
        )
        .unwrap();
        Self {
            _id: article_short._id.clone(),
            doi: article_short.doi.clone(),
            source: article_short.source.clone(),
            publication_date,
            title: article_short.title.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ParseError;

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mandatory json parsing could not be performed.")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    #[test]
    fn url_parse() {
        let obj_one = json!({
            "format": json!("html"),
            "platform": json!("web"),
            "value": json!("link0"),
        });
        let obj_two = json!({
            "format": json!("html"),
            "platform": json!("web"),
            "value": json!("link1"),
        });
        let obj_three = json!({
            "format": json!("html"),
            "platform": json!("web"),
            "value": json!(1.5),
        });

        let urls = json!([obj_one, obj_two, obj_three]);
        let out = urls
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| x.get("value"))
            .filter_map(|x| x.as_str())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(out[0], "link0");
        assert_eq!(out[1], "link1");
        assert_eq!(out.len(), 2);

        let obj_one = json!({
            "format": json!("html"),
            "platform": json!("web"),
            "value": json!(true),
        });
        let obj_two = json!({
            "format": json!("html"),
            "platform": json!("web"),
            "value": json!(1),
        });
        let obj_three = json!({
            "format": json!("html"),
            "platform": json!("web"),
            "value": json!(1.5),
        });
        let urls = json!([obj_one, obj_two, obj_three]);
        let out = urls
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| x.get("value"))
            .filter_map(|x| x.as_str())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(out.len(), 0);
    }
}
