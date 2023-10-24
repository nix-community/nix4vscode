use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::jinja::{Generator, SystemContext};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Extension {
    pub publisher_name: String,
    pub extension_name: String,
    pub asset_url: Option<String>,
    pub system: Option<SystemContext>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub vscode_version: String,
    pub autogen_warning: Option<String>,
    pub extensions: Vec<Extension>,
    pub system: Option<SystemContext>,
}

impl Config {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        let mut obj: Config = toml::from_str(fs::read_to_string(path).await?.as_str())?;

        obj.extensions.iter_mut().for_each(|item| {
            if item.system.is_none() {
                item.system = obj.system.clone();
            }
        });

        obj.extensions.iter_mut().for_each(|item| {
            if item.publisher_name.as_str() == "vadimcn"
                && item.extension_name.as_str() == "vscode-lldb"
            {
                item.asset_url = Some(Generator::CODELLDB.1.into());
            }
        });

        Ok(obj)
    }

    #[inline]
    fn get_idx(&self, publisher_name: &str, extension_name: &str) -> Option<usize> {
        self.extensions.iter().position(|item| {
            item.extension_name.as_str() == extension_name
                && item.publisher_name.as_str() == publisher_name
        })
    }

    pub fn get_system_ctx(
        &self,
        publisher_name: &str,
        extension_name: &str,
    ) -> Option<SystemContext> {
        match self.get_idx(publisher_name, extension_name) {
            Some(idx) => self.extensions[idx].system.clone(),
            None => None,
        }
    }

    pub fn get_asset_url(&self, publisher_name: &str, extension_name: &str) -> Option<String> {
        match self.get_idx(publisher_name, extension_name) {
            Some(idx) => self.extensions[idx].asset_url.clone(),
            None => None,
        }
    }

    pub fn contains(&self, publisher_name: &str, extension_name: &str) -> bool {
        self.get_idx(publisher_name, extension_name).is_some()
    }
}
