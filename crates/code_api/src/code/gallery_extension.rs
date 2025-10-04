mod extension_version;
mod version;
use std::fmt::Display;

pub use extension_version::*;
pub use version::*;

use super::{ResultMetaData, TargetPlatform};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryQueryResult {
    pub results: Vec<IRawGalleryExtensionsResult>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionStatistics {
    pub statistic_name: String,
    pub value: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionPublisher {
    pub display_name: String,
    /// In openvsx, this maybe empty.
    pub publisher_id: Option<String>,
    pub publisher_name: String,
    pub domain: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtension {
    pub extension_id: Option<String>,
    pub extension_name: String,
    pub display_name: Option<String>,
    pub short_description: Option<String>,
    pub publisher: IRawGalleryExtensionPublisher,
    pub versions: Vec<IRawGalleryExtensionVersion>,
    pub statistics: Vec<IRawGalleryExtensionStatistics>,
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionsResult {
    pub extensions: Vec<IRawGalleryExtension>,
    pub result_metadata: Vec<ResultMetaData>,
}

impl IRawGalleryExtensionsResult {
    pub fn get_target_platform(&self) -> Vec<TargetPlatform> {
        match self
            .result_metadata
            .iter()
            .position(|item| &item.metadata_type == "TargetPlatforms")
        {
            Some(idx) => {
                let _ = &self.result_metadata[idx];
                self.result_metadata[idx]
                    .metadata_items
                    .iter()
                    .map(|item| {
                        let a: TargetPlatform = item.name.as_str().into();
                        a
                    })
                    .collect()
            }
            None => vec![],
        }
    }
}
