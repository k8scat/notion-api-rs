use super::client::Client;
use anyhow::{Error, Result};
use crate::notion::common::RichText;
use crate::notion::page::Parent;

pub struct Database {
    pub title: Vec<RichText>,
    pub parent: Parent
}

impl Client {
    pub async fn query_database(&self, id: String) -> Result<Database> {
        Err(Error::msg(""))
    }

    pub async fn create_database(&self, id: String) -> Result<()> {
        Ok(())
    }

    pub async fn update_database(&self, id: String) -> Result<()> {
        Ok(())
    }

    pub async fn retrieve_database(&self, id: String) -> Result<Database> {
        Err(Error::msg(""))
    }
}