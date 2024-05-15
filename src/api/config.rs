use crate::storage::set_user_info;
use super::users::users;
use super::token::{get_token, set_token};
use dialoguer::{Select, theme::ColorfulTheme};
use console::Term;
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

    let user_nodes = users().await?;
    let user_names: Vec<&str> = user_nodes.iter().map(|u| u.name.as_str()).collect();

    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a user")
        .items(&user_names)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    if let Some(index) = user_selection {
        let selected_user = &user_nodes[index];
        set_user_info(selected_user.id.clone(), selected_user.name.clone())?;
        println!("Configuration saved. User: {}", selected_user.name);
    } else {
        println!("No user selected.");
    }

    Ok(())
}
