use super::*;
use vscode_derive::api;

#[api(Default)]
pub struct Extension {
    pub extension_id: String,
    pub extension_name: String,
    pub display_name: String,
    pub short_description: String,
    pub publisher: ExtensionPublisher,
    pub versions: Vec<ExtensionVersion>,
    pub statistics: Vec<ExtensionStatistics>,
    pub tags: Vec<String>,
    pub release_date: String,
    pub published_date: String,
    pub last_updated: String,
    pub categories: Vec<String>,
    pub flags: String,
}
