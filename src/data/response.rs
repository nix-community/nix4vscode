mod file;
use serde::{Deserialize, Serialize};

pub use file::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryQueryResult {
    pub results: Vec<IRawGalleryExtensionsResult>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionStatistics {
    pub statistic_name: String,
    pub value: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionPublisher {
    pub display_name: String,
    pub publisher_id: String,
    pub publisher_name: String,
    pub domain: Option<String>,
    pub is_domain_verified: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtension {
    pub extension_id: String,
    pub extension_name: String,
    pub display_name: String,
    pub short_description: Option<String>,
    pub publisher: IRawGalleryExtensionPublisher,
    pub versions: Vec<IRawGalleryExtensionVersion>,
    pub statistics: Vec<IRawGalleryExtensionStatistics>,
    pub tags: Option<String>,
    pub release_date: String,
    pub published_date: String,
    pub last_updated: String,
    pub categories: Option<Vec<String>>,
    pub flags: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionsResult {
    pub extensions: Vec<IRawGalleryExtension>,
}
