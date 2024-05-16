use serde::{Deserialize, Serialize};
use serde_json::json;
use super::client;
use anyhow::{Result, Context};

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

#[derive(Serialize, Deserialize, Debug)]
struct Me {
    id: String,
    name: String,
    email: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Viewer {
    viewer: Me,
}

#[derive(Serialize, Deserialize, Debug)]
struct MeResponse {
    data: Viewer,
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

async fn me() -> Result<Me, anyhow::Error> {
    let query = json!({
        "query": r#"
            query Me {
                viewer {
                    id
                    name
                    email
                }
            }
        "#
    });

    let response = client::make_request(&query.to_string()).await?;
    let me_response: MeResponse = serde_json::from_str(&response)
        .context("Failed to parse JSON response")?;

    Ok(me_response.data.viewer)
}

pub async fn print_me() -> Result<()> {
    let me = me().await?;

    println!("You are: ");
    println!("Name: {}", me.name);
    println!("Email: {}", me.email);
    println!("ID: {}", me.id);

    Ok(())
}