use std::fmt::Display;

use vscode_derive::api;

use super::*;

#[api(Default)]
pub struct IRawGalleryExtensionVersion {
    version: String,
    last_updated: String,
    asset_uri: String,
    fallback_asset_uri: String,
    files: Vec<IRawGalleryExtensionFile>,
    properties: Vec<IRawGalleryExtensionProperty>,
    target_platform: Option<String>,
}

impl Display for IRawGalleryExtensionVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl IRawGalleryExtensionVersion {
    pub fn get_download_asset(&self) -> IGalleryExtensionAsset {
        let target = match &self.target_platform {
            Some(target) => format!("&targetPlatform=${}", target),
            None => "".to_string(),
        };

        IGalleryExtensionAsset {
            uri: format!(
                "{}/{}?redirect=true{}",
                self.fallback_asset_uri,
                AssetType::Vsix.to_string(),
                target
            ),
            fallback_uri: format!(
                "{}/{}{}",
                self.fallback_asset_uri,
                AssetType::Vsix.to_string(),
                target
            ),
        }
    }

    pub fn get_version_asset(&self, ty: String) -> Option<IGalleryExtensionAsset> {
        let result = self.files.iter().any(|item| item.asset_type == ty);
        if !result {
            return None;
        }

        let target = match &self.target_platform {
            Some(target) => format!("&targetPlatform=${}", target),
            None => "".to_string(),
        };
        Some(IGalleryExtensionAsset {
            uri: format!("{}/{}{}", self.asset_uri, ty, target),
            fallback_uri: format!("{}/{}{}", self.fallback_asset_uri, ty, target),
        })
    }

    pub fn get_engine(&self) -> &str {
        match self
            .properties
            .iter()
            .find(|item| item.key == PropertyType::Engine.to_string())
        {
            Some(item) => &item.value,
            None => "",
        }
    }

    pub fn is_pre_release_version(&self) -> bool {
        self.properties
            .iter()
            .any(|item| item.key == PropertyType::PreRelease.to_string() && item.value == "true")
    }

    pub fn get_target_platform(&self) -> TargetPlatform {
        match self.target_platform {
            Some(ref platform) => platform.as_str().into(),
            None => TargetPlatform::Undefined,
        }
    }
}

#[api(Default)]
pub struct IGalleryExtensionAsset {
    uri: String,
    fallback_uri: String,
}
