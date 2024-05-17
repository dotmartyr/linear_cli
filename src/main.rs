use anyhow::Result;
use clap::{App, Command};

mod api;
mod storage;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new("Linear CLI")
        .version("0.1.0")
        .author("Erik Sandfort <eriksandfort@gmail.com>")
        .about("CLI for interacting with the Linear.app API")
        .subcommand(Command::new("config").about("Configure your API token and user for Linear"))
        .subcommand(Command::new("me").about("Display the current user's name"))
        .subcommand(Command::new("teams").about("List all teams from Linear"))
        .subcommand(
            Command::new("team:select").about("Select a team for context on further commands."),
        )
        .subcommand(Command::new("issues:ready").about("List your issues with 'Ready' state"))
        .subcommand(
            Command::new("issues:active").about("List your issues with 'In Progress' state"),
        )
        .subcommand(Command::new("issue:selected").about("Display details of the selected issue"))
        .subcommand(
            Command::new("issue:selected:clear").about("Clear the currently selected issue"),
        )
        .get_matches();

    match app.subcommand() {
        Some(("config", _)) => api::config::configure().await,
        Some(("me", _)) => api::users::print_me().await,
        Some(("teams", _)) => api::teams::print_teams().await,
        Some(("team:select", _)) => api::teams::select_team().await,
        Some(("issues:ready", _)) => api::issues::select_issue(Some("Ready")).await,
        Some(("issues:active", _)) => api::issues::select_issue(Some("In Progress")).await,
        Some(("issue:selected", _)) => api::issues::print_selected_issue().await,
        Some(("issue:selected:clear", _)) => {
            storage::clear_app_info(storage::ClearOption::Issue)?;
            Ok(())
        }

        _ => {
            eprintln!("Unknown command. Use '--help' for more information.");
            Ok(())
        }
    }
}
