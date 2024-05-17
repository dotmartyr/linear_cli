use super::client;
use crate::storage::set_team_info;
use anyhow::{Context, Result};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct TeamNode {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct TeamsNodes {
    nodes: Vec<TeamNode>,
}

#[derive(Deserialize, Debug)]
struct TeamsData {
    teams: TeamsNodes,
}

#[derive(Deserialize, Debug)]
struct TeamsResponse {
    data: TeamsData,
}

pub async fn teams() -> Result<Vec<TeamNode>> {
    let query = json!({
        "query": super::graphql_queries::TEAMS,
    });

    let response = client::make_request(&query).await?;
    let teams_response: TeamsResponse =
        serde_json::from_str(&response).context("Failed to parse JSON response")?;
    Ok(teams_response.data.teams.nodes)
}

pub async fn print_teams() -> Result<()> {
    let team_nodes = teams().await?;

    println!("Your Teams:");
    for team in team_nodes {
        println!("{}", team.name);
    }

    Ok(())
}

pub async fn select_team() -> Result<()> {
    let team_nodes = teams().await?;
    let team_names: Vec<&str> = team_nodes.iter().map(|team| team.name.as_str()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a team")
        .items(&team_names)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    if let Some(index) = selection {
        let selected_team = &team_nodes[index];
        set_team_info(selected_team.id.clone(), selected_team.name.clone())?;
        println!("Team selected: {}", selected_team.name);
    } else {
        println!("No team selected.");
    }

    Ok(())
}
