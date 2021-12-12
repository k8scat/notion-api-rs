use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use anyhow::{Result, Error};
use serde::Serialize;

const API_V1: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2021-08-16";

#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub base_api: String,
}

pub fn new(token: String) -> Result<Client> {
    if token.is_empty() {
        return Err(Error::msg("invalid token"));
    }
    let headers = HeaderMap::from_iter(vec![
        (HeaderName::from_str("Authorization").unwrap(), HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap()),
        (HeaderName::from_str("Notion-Version").unwrap(), HeaderValue::from_str(NOTION_VERSION).unwrap()),
    ]);
    let client = reqwest::ClientBuilder::new().default_headers(headers).build().unwrap();
    Ok(Client {
        client,
        base_api: API_V1.to_string(),
    })
}
