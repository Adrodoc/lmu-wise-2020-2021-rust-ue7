use std::fmt::Display;

use reqwest::Error;
use serde_json::Value;
use tokio::try_join;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<(), String> {
    let serde = print_keywords("serde");
    let tokio = print_keywords("tokio");
    let reqwest = print_keywords("reqwest");
    try_join!(serde, tokio, reqwest)?;
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
            .filter_map(|it| {
                if let Value::String(string) = it {
                    Some(string.as_str())
                } else {
                    None
                }
            })
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
