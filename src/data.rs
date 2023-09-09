// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensionManagement/common/extensionGalleryService.ts
mod packages;
mod response;

pub use packages::*;
pub use response::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NixContext {
    pub extension_name: String,
    pub publisher_name: String,
    pub extension_version: String,
    pub asset_url: Option<String>,
    pub sha256: String,
}
