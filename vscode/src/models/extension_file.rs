use vscode_derive::api;

#[api(Default)]
pub struct ExtensionFile {
    pub asset_type: String,
    pub source: String,
}
