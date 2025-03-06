use anyhow::anyhow;
use derive::api;
use itertools::Itertools;

use crate::code::PropertyType;

use super::*;

#[api]
pub struct IRawGalleryExtensionVersion {
    pub version: String,
    pub last_updated: String,
    pub asset_uri: String,
    pub fallback_asset_uri: String,
    pub files: Vec<IRawGalleryExtensionFile>,
    pub properties: Vec<IRawGalleryExtensionProperty>,
    pub target_platform: Option<String>,
}

impl Display for IRawGalleryExtensionVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl IRawGalleryExtensionVersion {
    pub fn get_engine(&self) -> anyhow::Result<String> {
        match self
            .properties
            .iter()
            .position(|item| item.key == PropertyType::Engine.to_string())
        {
            Some(idx) => Ok(self.properties[idx].value.clone()),
            None => Err(anyhow!("Missing attribute: engine")),
        }
    }

    pub fn is_pre_release_version(&self) -> bool {
        let values = self
            .properties
            .iter()
            .filter(|item| item.key == PropertyType::PRE_RELEASE)
            .collect_vec();

        !values.is_empty() && values[0].value == "true"
    }

    pub fn get_file(&self, file_kind: AssetType) -> Option<&IRawGalleryExtensionFile> {
        match self
            .files
            .iter()
            .position(|item| item.asset_type == file_kind.to_string())
        {
            Some(idx) => Some(&self.files[idx]),
            None => None,
        }
    }
}
