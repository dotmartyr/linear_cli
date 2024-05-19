use super::client;
use crate::api::config::read_team_id_from_config;
use anyhow::{Context, Result};
use console::{Style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    id: String,
    title: String,
    description: String,
    state: IssueState,
    team: IssueTeam,
    comments: CommentNodes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueNode {
    id: String,
    title: String,
    state: IssueState,
    team: IssueTeam,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssueState {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssueTeam {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuesNodes {
    nodes: Vec<IssueNode>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuesData {
    issues: IssuesNodes,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssueData {
    issue: Issue,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuesResponse {
    data: IssuesData,
}

#[derive(Serialize, Deserialize, Debug)]
struct IssueResponse {
    data: IssueData,
}

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    id: String,
    body: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommentNodes {
    nodes: Vec<Comment>,
}

pub async fn issues(user_id: &str, state_name: Option<&str>) -> Result<Vec<IssueNode>> {
    let team_id = read_team_id_from_config()?;

    let mut variables = serde_json::json!({
        "userId": user_id,
    });

    if let Some(sname) = state_name {
        variables["stateName"] = serde_json::json!(sname);
    }

    if let Some(t_id) = team_id {
        variables["teamId"] = serde_json::json!(t_id);
    }

    let query = json!({
        "query": super::graphql_queries::ISSUES,
        "variables": variables
    });

    let response = client::make_request(&query).await?;
    let issues_response: IssuesResponse =
        serde_json::from_str(&response).context("Failed to parse JSON response")?;
    Ok(issues_response.data.issues.nodes)
}

pub async fn issue(issue_id: &str) -> Result<Issue> {
    let query = json!({
        "query": super::graphql_queries::ISSUE,
        "variables": {
            "issueId": issue_id
        }
    });

    let response = client::make_request(&query).await?;
    let issue_response: IssueResponse =
        serde_json::from_str(&response).context("Failed to parse JSON response")?;
    Ok(issue_response.data.issue)
}

pub async fn select_issue(state_name: Option<&str>) -> Result<()> {
    let me = super::users::me().await?;
    loop {
        let issue_nodes = issues(&me.id, state_name).await?;
        if issue_nodes.is_empty() {
            println!("No issues found.");
            break;
        }

        if let Some(selected_issue) = handle_user_selection(&issue_nodes).await? {
            print_issue_details(&selected_issue.id).await?;
        } else {
            println!("No issue selected.");
            break;
        }
    }
    Ok(())
}

async fn handle_user_selection(issue_nodes: &[IssueNode]) -> Result<Option<&IssueNode>> {
    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an issue to view details")
        .items(
            &issue_nodes
                .iter()
                .map(|issue| format!("{} - {}", issue.team.name, issue.title))
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    Ok(selection_index.map(|index| &issue_nodes[index]))
}

async fn print_issue_details(issue_id: &str) -> Result<()> {
    let detailed_issue = issue(issue_id).await?;
    let blue_bold = Style::new().blue().bold();

    println!("{}", blue_bold.apply_to("Issue Details:"));
    println!(
        "{} {}",
        blue_bold.apply_to("Team Name:"),
        detailed_issue.team.name
    );
    println!("{} {}", blue_bold.apply_to("Title:"), detailed_issue.title);
    println!(
        "{} {}",
        blue_bold.apply_to("Description:"),
        detailed_issue.description
    );

    let _ = print_comments(&detailed_issue.comments).await;

    Ok(())
}

async fn print_comments(comments: &CommentNodes) -> Result<()> {
    let blue_bold = Style::new().blue().bold();
    println!("{}", blue_bold.apply_to("\nComments:"));
    for comment in &comments.nodes {
        println!("==============");
        println!(" - {}: {}", comment.created_at, comment.body);
    }
    Ok(())
}
