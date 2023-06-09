// API key required to make requests to Springer API I generated it without
// problems but it's not clear to me how it works, and if it's all free or not.

// Function used to return parsed url object.
pub fn springer_articles_url(
    subject: &str,
    article_type: &str,
    from_date: &chrono::NaiveDate,
    till_date: &chrono::NaiveDate,
    idx: usize,
    amount: usize,
) -> reqwest::Url {
    // For now its hard-coded because I was testing how the API is supposed to work.
    let url = format!(
        "https://api.springernature.com/meta/v2/json?api_key={}&q=subject:{}+type:{}+onlinedatefrom:{}+onlinedateto:{}+sort:date&s={}&p={}",
        std::env::var("SPRINGER_API_KEY").expect("SPRINGER_API_KEY must be set."), 
        subject,
        article_type,
        from_date.to_string(),
        till_date.to_string(),
        idx,
        amount,
    );
    reqwest::Url::parse(&url).unwrap()
}

pub async fn springer_json_response(
    client: &reqwest::Client,
    springer_articles_url: reqwest::Url,
) -> Result<serde_json::Value, reqwest::Error> {
    client
        .get(springer_articles_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await
}
