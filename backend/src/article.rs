use chrono::{Datelike, NaiveDate};
use mongodb::bson::{self, doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    _id: String,
    source: String,
    publication_date: bson::datetime::DateTime,
    title: String,
    authors: Vec<String>,
    key_words: Vec<String>,
    urls: Vec<String>,
    article_abstract: String,
}

#[derive(Debug)]
pub struct ParseError;

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mandatory json parsing could not be performed.")
    }
}

impl Article {
    pub fn from_springer(springer_record: &serde_json::Value) -> Result<Self, ParseError> {
        // mandatory
        let _id = springer_record
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
        Ok(Article {
            _id,
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
