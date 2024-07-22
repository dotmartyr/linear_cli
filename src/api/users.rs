use super::client;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserNode {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeResponse {
    pub data: Viewer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Viewer {
    pub viewer: Me,
}

impl Me {
    pub fn print(&self) {
        println!("You are: ");
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("ID: {}", self.id);
    }
}

pub async fn me() -> Result<Me, anyhow::Error> {
    let query = json!({
        "query": super::graphql_queries::ME,
    });

    let response = client::make_request(&query).await?;
    let me_response: MeResponse =
        serde_json::from_str(&response).context("Failed to parse JSON response")?;

    Ok(me_response.data.viewer)
}

pub async fn print_me() -> Result<()> {
    let me = me().await?;
    me.print();
    Ok(())
}
