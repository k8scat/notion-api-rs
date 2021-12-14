use super::client::Client;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub object: String,
    pub id: String,
    pub block_type: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub archived: bool,
    pub has_children: bool
}

impl Client {
    pub async fn retrieve_block(&self, id: String) -> Result<()> {
        Ok(())
    }

    pub async fn update_block(&self, id: String, block: Block) -> Result<()> {
        Ok(())
    }

    pub async fn delete_block(&self, id: String) -> Result<()> {
        Ok(())
    }
}