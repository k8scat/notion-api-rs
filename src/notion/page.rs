use super::client::Client;
use super::common::File;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub object: String,
    pub id: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub archived: bool,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub url: String,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    pub parent_type: String,
    pub database_id: Option<String>,
    pub page_id: Option<String>,
    pub workspace: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "type")]
    pub icon_type: String,
    pub file: Option<File>,
    pub emoji: Option<String>,
}

impl Client {
    pub async fn retrieve_page(&self, id: String) -> Result<Page> {
        let url = format!("{}/pages/{}", self.base_api, id);
        let resp = self.client.get(&url).send().await?;
        let page = resp.json::<Page>().await?;
        Ok(page)
    }

    pub async fn create_page(&self) -> Result<()> {
        Ok(())
    }

    pub async fn update_page(&self) -> Result<()> {
        Ok(())
    }
}