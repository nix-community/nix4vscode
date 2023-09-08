use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vscode_version: String,
    pub extensions: Vec<Extension>,
}

impl Config {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        Ok(toml::from_str(fs::read_to_string(path).await?.as_str())?)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Extension {
    pub publisher_name: String,
    pub extension_name: String,
}
