use serde::{Deserialize, Serialize};

mod asset_url;

pub use asset_url::*;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeneratorContext {
    pub autogen_warning: Option<String>,
    pub extensions: Vec<NixContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixContext {
    pub extension_name: String,
    pub publisher_name: String,
    pub extension_version: String,
    pub asset_url: Option<String>,
    pub sha256: String,
}
