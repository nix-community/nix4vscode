mod extension_version;
mod version;
use std::fmt::Display;

use derive::api;

pub use extension_version::*;
pub use version::*;

use super::{ResultMetaData, TargetPlatform};

#[api]
pub struct IRawGalleryQueryResult {
    pub results: Vec<IRawGalleryExtensionsResult>,
}

#[api]
pub struct IRawGalleryExtensionPublisher {
    pub publisher_name: String,
}

#[api]
pub struct IRawGalleryExtension {
    pub extension_name: String,
    pub publisher: IRawGalleryExtensionPublisher,
    pub versions: Vec<IRawGalleryExtensionVersion>,
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

#[api]
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
