use reqwest::Error;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<(), String> {
    let body = fetch_crateinfo("reqwest").await?;
    println!("body = {:?}", body);
    Ok(())
}
async fn crate_keywords(crate_name: &str) -> Result<String, String> {
    let crate_info = fetch_crateinfo(crate_name).await?;
    keywords_from_response(crate_info)
}

fn keywords_from_response(response: String) -> Result<String, String> {
    Ok("".to_string())
}

async fn fetch_crateinfo(crate_name: &str) -> Result<String, String> {
    let url = "https://crates.io/api/v1/crates/".to_owned() + crate_name;
    let result = fetch_url(&url).await;
    result.map_err(|error| format!("error code: {}", error))
}

async fn fetch_url(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let response = client.get(url).send().await?;
    response.text().await
}
