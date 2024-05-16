use super::token::{get_token, set_token};
use anyhow::Result;

pub async fn configure() -> Result<()> {

    // Check if an API token is already set
    match get_token() {
        Ok(token) if !token.is_empty() => {
            println!("Using existing API token.");
        },
        _ => {
            // Prompt for a new token if not set or empty
            println!("No API token found or token is empty.");
            set_token()?;  // Set new token
        }
    }

    Ok(())
}
