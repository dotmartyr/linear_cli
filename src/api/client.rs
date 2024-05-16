use reqwest::{Client, header};
use serde_json::Value;
use anyhow::{Result, Context};

pub async fn make_request(payload: &Value) -> Result<String> {
    let token = super::token::get_token()?;

    let client = Client::new();
    let payload_owned = payload.to_string();  // Clone the payload to own it
    let res = client.post("https://api.linear.app/graphql")
        .header(header::AUTHORIZATION, token)
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload_owned)
        .send()
        .await
        .context("Failed to send request to Linear API")?;

    let status = res.status();
    if status.is_success() {
        res.text().await.context("Failed to read response text")
    } else {
        let error_text = res.text().await.context("Failed to read error response text")?;
        Err(anyhow::anyhow!("Error: Failed to fetch data. Status: {}, Body: {}", status, error_text))
    }
}
