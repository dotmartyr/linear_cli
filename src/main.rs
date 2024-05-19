use anyhow::Result;
use clap::{App, Command};

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new("Linear CLI")
        .version("0.1.0")
        .author("Erik Sandfort <eriksandfort@gmail.com>")
        .about("CLI for interacting with the Linear.app API")
        .subcommand(Command::new("config").about("Configure your API token and user for Linear"))
        .subcommand(
            Command::new("config:team")
                .about("Configure the team settings for the current directory"),
        )
        .subcommand(Command::new("me").about("Display your information"))
        .subcommand(
            Command::new("team").about("Display the team configured for the current directory"),
        )
        .subcommand(Command::new("teams").about("List all teams you are associated with"))
        .subcommand(Command::new("issues:ready").about("List your issues with 'Ready' state"))
        .subcommand(
            Command::new("issues:active").about("List your issues with 'In Progress' state"),
        )
        .get_matches();

    match app.subcommand() {
        Some(("config", _)) => api::config::configure_token().await,
        Some(("config:team", _)) => api::config::configure_team_for_directory().await,
        Some(("me", _)) => api::users::print_me().await,
        Some(("teams", _)) => api::teams::print_teams().await,
        Some(("team", _)) => api::teams::print_configured_team_info().await,
        Some(("issues:ready", _)) => api::issues::select_issue(Some("Ready")).await,
        Some(("issues:active", _)) => api::issues::select_issue(Some("In Progress")).await,
        _ => {
            eprintln!("Unknown command. Use '--help' for more information.");
            Ok(())
        }
    }
}
