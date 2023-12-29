use super::*;
use std::fmt::Display;
use vscode_derive::api;

use super::IRawGalleryExtensionVersion;

#[api(Default)]
pub struct IRawGalleryQueryResult {
    pub results: Vec<IRawGalleryExtensionsResult>,
}

#[api(Default)]
pub struct IRawGalleryExtensionStatistics {
    pub statistic_name: String,
    pub value: f64,
}

#[api(Default)]
pub struct IRawGalleryExtensionPublisher {
    pub display_name: String,
    pub publisher_id: String,
    pub publisher_name: String,
    pub domain: Option<String>,
    pub is_domain_verified: bool,
}

#[api(Default)]
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

impl Display for IRawGalleryExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.publisher.publisher_name, self.extension_name
        )
    }
}

#[api(Default)]
pub struct IRawGalleryExtensionsResult {
    pub extensions: Vec<IRawGalleryExtension>,
    pub result_metadata: Vec<ResultMetaData>,
}
