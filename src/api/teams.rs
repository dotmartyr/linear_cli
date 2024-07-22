use super::client;
use crate::api::config::read_team_id_from_config;
use anyhow::{Context, Result};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub id: String,
    pub name: String,
}

impl Team {
    pub fn print(&self) {
        println!("{}", self.name);
    }
}

#[derive(Deserialize, Debug)]
pub struct TeamsNodes {
    pub nodes: Vec<Team>,
}

#[derive(Deserialize, Debug)]
pub struct TeamsData {
    pub teams: TeamsNodes,
}

#[derive(Deserialize, Debug)]
pub struct TeamsResponse {
    pub data: TeamsData,
}

#[derive(Deserialize, Debug)]
pub struct TeamData {
    pub team: Team,
}

#[derive(Deserialize, Debug)]
pub struct TeamResponse {
    pub data: TeamData,
}

pub async fn teams() -> Result<Vec<Team>> {
    let query = json!({
        "query": super::graphql_queries::TEAMS,
    });

    let response = client::make_request(&query).await?;
    let teams_response: TeamsResponse =
        serde_json::from_str(&response).context("Failed to parse JSON response")?;
    Ok(teams_response.data.teams.nodes)
}

pub async fn team(team_id: &str) -> Result<Team> {
    let query = json!({
        "query": super::graphql_queries::TEAM,
        "variables": {
            "teamId": team_id
        }
    });

    let response = client::make_request(&query).await?;
    let team_response: TeamResponse =
        serde_json::from_str(&response).context("Failed to parse team JSON response")?;

    Ok(team_response.data.team)
}

pub async fn configured_team() -> Result<Option<Team>> {
    match read_team_id_from_config()? {
        Some(team_id) => Ok(Some(team(&team_id).await?)),
        None => Ok(None),
    }
}

pub async fn print_teams() -> Result<()> {
    let team_nodes = teams().await?;

    println!("Your Teams:");
    for team in team_nodes {
        team.print();
    }

    Ok(())
}

pub async fn print_configured_team_info() -> Result<()> {
    match configured_team().await {
        Ok(Some(team)) => {
            println!("Configured Team: {}", team.name);
        }
        Ok(None) => {
            println!("No configured team for this directory, please use the config:team command.");
        }
        Err(e) => {
            println!("Error retrieving configured team: {}", e);
            return Err(e);
        }
    }
    Ok(())
}

pub async fn select_team() -> Result<Team> {
    let team_nodes = teams().await?;
    let team_names: Vec<&str> = team_nodes.iter().map(|team| team.name.as_str()).collect();

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a team")
            .items(&team_names)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        if let Some(index) = selection {
            let selected_team = &team_nodes[index];
            println!("Team selected: {}", selected_team.name);
            return Ok(selected_team.clone());
        } else {
            println!("No team selected, please select a team.");
        }
    }
}
