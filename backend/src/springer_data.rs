// API key required to make requests to Springer API I generated it without 
// problems but it's not clear to me how it works, and if it's all free or not.
const API_KEY:& str = "23a7034c6c6d15d46c624d2fb5e003c9";

// Function used to return parsed url object.
fn bioinformatics_articles(start: usize, records: usize) -> reqwest::Url {
    // For now its hard-coded because I was testing how the API is supposed to work.
    let url = format!("https://api.springernature.com/meta/v2/json?api_key={API_KEY}&q=subject:Bioinformatics+type:Journal+onlinedatefrom:2023-04-01+onlinedateto:2023-04-30+sort:date&s={start}&p={records}");
    reqwest::Url::parse(&url).unwrap()
}

// Function for making the acutal request. Async for the future when we will be 
// possibly making much more requests
async fn request(client: &reqwest::Client) -> Result<reqwest::Response, reqwest::Error> {
    client.get(bioinformatics_articles(1, 100)).send().await
}

// this function makes requests and returns serialized value.
pub async fn load_data() -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = request(&client).await?;
    let body = res.text().await?;
    Ok(serde_json::from_str(&body).unwrap())
}
