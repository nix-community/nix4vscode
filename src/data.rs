// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensionManagement/common/extensionGalleryService.ts
mod packages;
mod response;

pub use packages::*;
pub use response::*;

#[derive(Debug)]
pub struct NixContext {
    pub extension_name: String,
    pub publisher_name: String,
    pub extension_version: String,
    pub asset_url: String,
    pub sha256: String,
}
