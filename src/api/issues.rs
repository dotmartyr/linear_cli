use super::client;
use crate::storage::{get_selected_issue, set_selected_issue};
use anyhow::{Context, Result};
use console::{Style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
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
    let mut variables = serde_json::json!({
        "userId": user_id,
    });

    if let Some(sname) = state_name {
        variables["stateName"] = serde_json::json!(sname);
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
        let issue_nodes = super::issues::issues(&me.id, state_name).await?;
        let issue_options: Vec<String> = issue_nodes
            .iter()
            .map(|issue| format!("{} - {}", issue.team.name, issue.title))
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("View Issue")
            .items(&issue_options)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(index) => {
                let selected_issue = &issue_nodes[index];
                print_issue_details(&selected_issue.id).await?;

                // Submenu after showing issue details
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Set as the selected issue?")
                    .interact()?
                {
                    set_selected_issue(selected_issue.id.clone(), selected_issue.title.clone())?;
                    println!("Issue set as selected.");
                    break; // Exit the function immediately after setting the issue
                }
                // No else needed, loop will continue automatically returning to the issue list
            }
            None => {
                println!("No issue selected.");
                break;
            }
        }
    }
    Ok(())
}

async fn print_issue_details(issue_id: &str) -> Result<()> {
    let detailed_issue = super::issues::issue(issue_id).await?;
    let blue_bold = Style::new().blue().bold();

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

    println!("{}", blue_bold.apply_to("\nComments:"));
    for comment in detailed_issue.comments.nodes {
        println!("==============");
        println!(" - {}: {}", comment.created_at, comment.body);
    }

    Ok(())
}

pub async fn print_selected_issue() -> Result<()> {
    if let Some(issue_info) = get_selected_issue() {
        print_issue_details(&issue_info.id).await
    } else {
        println!("No selected issue.");
        Ok(())
    }
}
