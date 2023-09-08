use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionProperty {
    key: String,
    value: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct IRawGalleryExtensionFile {
    pub asset_type: String,
    pub source: String,
}

pub enum AssetType {
    Icon,
    Details,
    Changelog,
    Manifest,
    Vsix,
    License,
    Repository,
    Signature,
}

impl ToString for AssetType {
    #[rustfmt::skip]
    fn to_string(&self) -> String {
        match self {
            Self::Icon=> "Microsoft.VisualStudio.Services.Icons.Default".into(),
            Self::Details=> "Microsoft.VisualStudio.Services.Content.Details".into(),
            Self::Changelog=> "Microsoft.VisualStudio.Services.Content.Changelog".into(),
            Self::Manifest=> "Microsoft.VisualStudio.Code.Manifest".into(),
            Self::Vsix=> "Microsoft.VisualStudio.Services.VSIXPackage".into(),
            Self::License=> "Microsoft.VisualStudio.Services.Content.License".into(),
            Self::Repository=> "Microsoft.VisualStudio.Services.Links.Source".into(),
            Self::Signature=> "Microsoft.VisualStudio.Services.VsixSignature".into()
        }
    }
}

impl IRawGalleryExtensionVersion {
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
