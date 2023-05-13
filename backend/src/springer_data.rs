// API key required to make requests to Springer API I generated it without
// problems but it's not clear to me how it works, and if it's all free or not.

use chrono::NaiveDate;

// Function used to return parsed url object.
fn springer_articles_url(subject: &str, article_type: &str, from_date: &NaiveDate, till_date: &NaiveDate, idx: usize, amount: usize) -> reqwest::Url {
    // For now its hard-coded because I was testing how the API is supposed to work.
    let url = format!(
        "https://api.springernature.com/meta/v2/json?api_key={}&q=subject:{}+type:{}+onlinedatefrom:{}+onlinedateto:{}+sort:date&s={}&p={}",
        std::env::var("SPRINGER_API_KEY").expect("SPRINGER_API_KEY must be set."), 
        subject,
        article_type,
        from_date.to_string(),
        till_date.to_string(),
        idx, 
        amount
    );
    reqwest::Url::parse(&url).unwrap()
}

// Function for making the acutal request. Async for the future when we will be
// possibly making much more requests
async fn request(client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    let from_date = NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
    let till_date = NaiveDate::from_ymd_opt(2023, 4, 30).unwrap();
    client.get(springer_articles_url("Bioinformatics", "Journal", &from_date, &till_date, 1, 100)).send().await
}

// this function makes requests and returns serialized value.
pub async fn load_data() -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = request(&client).await?;
    let body = res.text().await?;
    Ok(serde_json::from_str(&body).unwrap())
}
