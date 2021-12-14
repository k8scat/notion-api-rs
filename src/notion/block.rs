use super::client::Client;
use anyhow::Result;

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
    pub async fn retrieve_block(&self, id: String) -> Result<Block> {
        let url = format!("{}/api/v3/blocks/{}", self.base_url, id);
        let response = self.get(&url).await?;
        let body = response.text().await?;
        let block: Block = serde_json::from_str(&body)?;
        Ok(block)
    }

    pub async fn update_block(&self, id: String, block: Block) -> Result<Block> {
        let url = format!("{}/api/v3/blocks/{}", self.base_url, id);
        let response = self.put(&url, &block).await?;
        let body = response.text().await?;
        let block: Block = serde_json::from_str(&body)?;
        Ok(block)
    }

    pub async fn delete_block(&self, id: String) -> Result<()> {
        Ok(())
    }
}