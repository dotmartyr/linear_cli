use anyhow::{Context, Result};
use reqwest::{Client, header};
use serde_json::Value;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn make_request(payload: &Value) -> Result<String> {
    let token = super::token::get_token()?;
    
    let client = CLIENT.get_or_init(|| Client::new());
    let res = client.post("https://api.linear.app/graphql")
        .header(header::AUTHORIZATION, token)
        .header(header::CONTENT_TYPE, "application/json")
        .json(payload)
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
