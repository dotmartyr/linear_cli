use super::teams::select_team;
use super::token::{get_token, set_token};
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub async fn configure_token() -> Result<()> {
    // Check if an API token is already set
    match get_token() {
        Ok(token) if !token.is_empty() => {
            println!("Using existing API token.");
        }
        _ => {
            println!("No API token found or token is empty.");
            set_token()?;
        }
    }

    Ok(())
}

pub async fn configure_team_for_directory() -> Result<()> {
    let selected_team = select_team().await?;

    // Create the .config directory if it doesn't exist
    let config_dir = Path::new(".config");
    fs::create_dir_all(config_dir).context("Failed to create .config directory")?;

    // Create or overwrite the linear.json file
    let file_path = config_dir.join("linear.json");
    let mut file = File::create(&file_path).context("Failed to create linear.json file")?;

    // Write the team ID to the file
    let contents = format!(r#"{{"teamId": "{}"}}"#, selected_team.id);
    file.write_all(contents.as_bytes())
        .context("Failed to write to linear.json")?;

    println!("Team configuration saved in {}", file_path.display());

    Ok(())
}

pub fn read_team_id_from_config() -> Result<Option<String>> {
    let file_path = ".config/linear.json";
    match fs::read_to_string(file_path) {
        Ok(config_contents) => {
            let config: Value = serde_json::from_str(&config_contents)?;
            if let Some(team_id) = config["teamId"].as_str() {
                Ok(Some(team_id.to_string()))
            } else {
                Ok(None) // Team ID not found in config, but config exists
            }
        }
        Err(_) => {
            Ok(None) // Config file does not exist
        }
    }
}
