use eyre::eyre;
use eyre::Result;
use serde_json::Value;

pub async fn fetch(url: &str) -> Result<Value> {
    let body = reqwest::get(url).await?.text().await?;
    return match serde_json::from_str(&body) {
        Ok(v) => Ok(v),
        Err(e) => Err(eyre!("Could not fetch data: {}", e)),
    };
}

