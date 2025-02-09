use std::sync::Arc;

use code_api::code::TargetPlatform;
use serde::{Deserialize, Serialize};

mod asset_url;

pub use asset_url::*;

use crate::config::Config;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeneratorContext {
    pub config: Arc<Config>,
    pub extensions: Vec<NixContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixContext {
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub asset_url: Option<String>,
    pub sha256: String,
    pub platform: TargetPlatform,
}
