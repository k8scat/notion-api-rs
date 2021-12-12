use super::client::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    object: String,
    results: Vec<User>,
    next_cursor: Option<String>,
    has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub object: String,
    pub id: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<String>,
    pub person: Option<Person>,
    pub bot: Option<Bot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "type")]
    pub owner_type: String,
    pub workspace: Option<bool>,
    pub user: Box<Option<User>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub email: String
}

impl Client {
    pub async fn list_users(&self) -> Result<Vec<User>> {
        let url = format!("{}/users", self.base_api);
        let resp = self.client.get(url).send().await?;
        let data = resp.json::<ListUsersResponse>().await?;
        Ok(data.results)
    }

    pub async fn get_user(&self, id: String) -> Result<User> {
        let url = format!("{}/users/{}", self.base_api, id);
        let resp = self.client.get(url).send().await?;
        let user = resp.json::<User>().await?;
        Ok(user)
    }

    pub async fn me(&self) -> Result<User> {
        self.get_user(String::from("me")).await
    }
}
