use anyhow::anyhow;
use lazy_regex::regex;
use serde::{Deserialize, Serialize};

use crate::jinja::{Generator, SystemContext};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Extension {
    pub publisher_name: String,
    pub extension_name: String,
    pub asset_url: Option<String>,
    pub system: Option<SystemContext>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub vscode_version: String,
    pub autogen_warning: Option<String>,
    pub extensions: Vec<Extension>,
    pub system: Option<SystemContext>,
}

impl Config {
    pub fn new(content: &str) -> anyhow::Result<Self> {
        let mut obj: Config = toml::from_str(content)?;
        let reg = regex!(r#"(\d+.\d+.\d+)(.*)?"#)
            .captures(&obj.vscode_version)
            .ok_or(anyhow!(format!("bad code version: {}", obj.vscode_version)))
            .unwrap();
        assert_eq!(3, reg.len());

        obj.vscode_version = reg[1].to_string();

        obj.extensions.iter_mut().for_each(|item| {
            if item.system.is_none() {
                item.system.clone_from(&obj.system);
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
            item.extension_name.to_lowercase() == extension_name.to_lowercase()
                && item.publisher_name.to_lowercase() == publisher_name.to_lowercase()
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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_version() {
        let c = [
            r#"vscode_version = "0.1.23""#,
            r#"vscode_version = "101.20.0""#,
            r#"vscode_version = "1.84.4""#,
            r#"vscode_version = "1.86.2.24057""#,
            r#"vscode_version = "1.84.4-preview""#,
            r#"vscode_version = "1.86.2.24057-preview""#,
        ];

        for i in c {
            let i = Config::new(i).unwrap();
            let _ = semver::Version::from_str(&i.vscode_version).unwrap();
        }
    }
}
