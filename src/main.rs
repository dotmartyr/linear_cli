use clap::{App, Command};
use anyhow::Result;

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
        .subcommand(Command::new("select-team").about("Select a team for context on further commands."))
        .get_matches();

    match app.subcommand() {
        Some(("config", _)) => api::config::configure().await,
        Some(("me", _)) => Ok(api::users::me()?),
        Some(("teams", _)) => api::teams::print_teams().await,
        Some(("select-team", _)) => api::teams::select_team().await,
        _ => {
            eprintln!("Unknown command. Use '--help' for more information.");
            Ok(())
        }
    }
}
