use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Extension {
    pub publisher_name: String,
    pub extension_name: String,
    pub asset_url: Option<String>,
    pub system: Option<SystemContext>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SystemContext {
    arch: String,
    ostype: String,
}

impl Default for SystemContext {
    fn default() -> Self {
        Self {
            arch: std::env::consts::ARCH.into(),
            ostype: std::env::consts::OS.into(),
        }
    }
}
