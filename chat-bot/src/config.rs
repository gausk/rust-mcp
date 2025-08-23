use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub openai_key: Option<String>,
    pub chat_url: Option<String>,
    pub model_name: Option<String>,
    pub support_tool: Option<bool>,
}

impl Config {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
