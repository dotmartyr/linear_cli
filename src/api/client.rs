use reqwest::{Client, header};
use anyhow::{Result, Context};

pub async fn make_request(payload: &str) -> Result<String> {
    let token = super::token::get_token()?;  // Ensure you have token handling in place

    let client = Client::new();
    let payload_owned = payload.to_string();  // Clone the payload to own it
    let res = client.post("https://api.linear.app/graphql")
        .header(header::AUTHORIZATION, token)
        .header(header::CONTENT_TYPE, "application/json")
        .body(payload_owned)  // Use the owned payload here
        .send()
        .await
        .context("Failed to send request to Linear API")?;

    // Capture the status before consuming the response body
    let status = res.status();

    // Conditionally access the body based on the status
    if status.is_success() {
        res.text().await.context("Failed to read response text")
    } else {
        let error_text = res.text().await.context("Failed to read error response text")?;
        Err(anyhow::anyhow!("Error: Failed to fetch data. Status: {}, Body: {}", status, error_text))
    }
}
