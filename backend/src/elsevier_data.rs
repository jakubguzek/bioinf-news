/// Function used to return parsed elsevier api url.
pub fn elsevier_articles_url(
    from_date: &chrono::NaiveDate,
    till_date: &chrono::NaiveDate,
) -> reqwest::Url {
    let url = format!(
        "https://api.elsevier.com/content/metadata/article?pub-date%20AFT%20{}%20AND%20pub-date%20BEF%20{}&apiKey={}",
        from_date,
        till_date,
        std::env::var("ELSEVIER_API_KEY").expect("ELSEVIER_API_KEY must be set.")
    );
    reqwest::Url::parse(&url).unwrap()
}

pub async fn elsevier_json_response(
    client: &reqwest::Client,
    elsevier_articles_url: reqwest::Url,
) -> Result<serde_json::Value, reqwest::Error> {
    client
        .get(elsevier_articles_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await
}
