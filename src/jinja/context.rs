use std::sync::Arc;

use derive::api;
use serde::{Deserialize, Serialize};

mod asset_url;

pub use asset_url::*;

use crate::{code::TargetPlatform, config::Config};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GeneratorContext {
    pub config: Arc<Config>,
    pub extensions: Vec<NixContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixContext {
    pub extension_name: String,
    pub publisher_name: String,
    pub extension_version: String,
    pub asset_url: Option<String>,
    pub sha256: String,
    pub target_platform: TargetPlatform,
}

#[api]
pub struct CodeExtension {
    publisher: String,
    name: String,
    universal: Option<CodeExt>,
    x86_linux: Option<CodeExt>,
    aarch64_linux: Option<CodeExt>,
    x86_darwin: Option<CodeExt>,
    aarch64_darwin: Option<CodeExt>,
}

#[api]
pub struct CodeExt {
    version: String,
    sha256: Option<String>,
}
