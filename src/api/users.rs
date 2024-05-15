use serde::{Deserialize, Serialize};
use serde_json::json;
use super::client;
use crate::storage::get_user_info;
use anyhow::{Result, Context, bail};


#[derive(Serialize, Deserialize, Debug)]
pub struct UserNode {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsersNodes {
    nodes: Vec<UserNode>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Users {
    users: UsersNodes,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsersResponse {
    data: Users,
}

pub async fn users() -> Result<Vec<UserNode>, anyhow::Error> {
    let query = json!({
        "query": r#"
            query {
              users {
                nodes {
                  name
                  id
                }
              }
            }
        "#
    });

    let response = client::make_request(&query.to_string()).await?;
    let users_response: UsersResponse = serde_json::from_str(&response)
        .context("Failed to parse JSON response")?;

    Ok(users_response.data.users.nodes)
}

pub fn me() -> Result<()> {
    if let Some(user) = get_user_info() {
        println!("Current user: {}", user.name);
        Ok(())
    } else {
        bail!("No current user set.");
    }
}