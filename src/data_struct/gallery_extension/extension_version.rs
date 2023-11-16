use serde::{Deserialize, Serialize};

use crate::{error::Error, request::PropertyType};

use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionVersion {
    pub version: String,
    pub last_updated: String,
    pub asset_uri: String,
    pub fallback_asset_uri: String,
    pub files: Vec<IRawGalleryExtensionFile>,
    pub properties: Vec<IRawGalleryExtensionProperty>,
    pub target_platform: String,
}

impl IRawGalleryExtensionVersion {
    pub fn get_engine(&self) -> anyhow::Result<semver::VersionReq> {
        match self
            .properties
            .iter()
            .position(|item| item.key == PropertyType::Engine.to_string())
        {
            Some(idx) => Ok(semver::VersionReq::parse(&self.properties[idx].value).unwrap()),
            None => Err(Error::AttributeMissing("engine".into()).into()),
        }
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
