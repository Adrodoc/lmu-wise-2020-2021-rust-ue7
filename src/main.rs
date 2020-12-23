use futures::future::try_join_all;
use reqwest::Error;
use serde_json::Value;
use std::fmt::Display;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<(), String> {
    let crate_names = ["serde", "tokio", "reqwest"];
    try_join_all(crate_names.iter().map(|it| print_keywords(it))).await?;
    Ok(())
}

async fn print_keywords(crate_name: &str) -> Result<(), String> {
    let keywords = crate_keywords(crate_name).await?;
    println!("keywords of {}: {}", crate_name, keywords);
    Ok(())
}

async fn crate_keywords(crate_name: &str) -> Result<String, String> {
    let crate_info = fetch_crateinfo(crate_name).await?;
    keywords_from_response(crate_info)
}

fn keywords_from_response(crate_info: String) -> Result<String, String> {
    let json: Value = serde_json::from_str(&crate_info).map_err(err_to_string)?;
    if let Value::Array(keywords) = &json["crate"]["keywords"] {
        let result = keywords
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        Ok(result)
    } else {
        Err("No keywords in body".to_string())
    }
}

async fn fetch_crateinfo(crate_name: &str) -> Result<String, String> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let result = fetch_url(&url).await;
    result.map_err(err_to_string)
}

fn err_to_string<E: Display>(error: E) -> String {
    format!("error: {}", error)
}

async fn fetch_url(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let response = client.get(url).send().await?;
    response.text().await
}
