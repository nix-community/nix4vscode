use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::jinja::Generator;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vscode_version: String,
    pub extensions: Vec<Extension>,
}

impl Config {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        let mut obj: Config = toml::from_str(fs::read_to_string(path).await?.as_str())?;

        obj.extensions.iter_mut().for_each(|item| {
            if item.publisher_name.as_str() == "vadimcn"
                && item.extension_name.as_str() == "vscode-lldb"
            {
                item.asset_url = Some(Generator::CODELLDB.1.into());
            }
        });
        Ok(obj)
    }

    pub fn get_asset_url(&self, publisher_name: &str, extension_name: &str) -> Option<String> {
        match self.extensions.iter().position(|item| {
            item.extension_name.as_str() == extension_name
                && item.publisher_name.as_str() == publisher_name
        }) {
            Some(idx) => self.extensions[idx].asset_url.clone(),
            None => None,
        }
    }

    pub fn contains(&self, publisher_name: &str, extension_name: &str) -> bool {
        self.extensions.iter().any(|item| {
            item.extension_name.as_str() == extension_name
                && item.publisher_name.as_str() == publisher_name
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Extension {
    pub publisher_name: String,
    pub extension_name: String,
    pub asset_url: Option<String>,
}
