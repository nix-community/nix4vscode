use vscode_derive::api;

use super::*;

#[api(Default)]
pub struct ExtensionVersion {
    pub version: String,
    pub last_updated: String,
    pub asset_uri: String,
    pub fallback_asset_uri: String,
    pub files: Vec<ExtensionFile>,
    pub properties: Vec<ExtensionProperty>,
    pub target_platform: String,
}
