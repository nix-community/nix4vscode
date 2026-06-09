mod extension_version;
mod version;
use std::fmt::Display;

pub use version::*;

use super::{IRawGalleryExtension, IRawGalleryExtensionsResult, TargetPlatform};

impl Display for IRawGalleryExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.publisher.publisher_name, self.extension_name
        )
    }
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
