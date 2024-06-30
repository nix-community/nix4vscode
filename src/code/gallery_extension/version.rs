use std::fmt::Display;

use derive::api;

#[api]
pub struct IRawGalleryExtensionProperty {
    pub key: String,
    pub value: String,
}

#[api]
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

impl Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Icon => "Microsoft.VisualStudio.Services.Icons.Default",
            Self::Details => "Microsoft.VisualStudio.Services.Content.Details",
            Self::Changelog => "Microsoft.VisualStudio.Services.Content.Changelog",
            Self::Manifest => "Microsoft.VisualStudio.Code.Manifest",
            Self::Vsix => "Microsoft.VisualStudio.Services.VSIXPackage",
            Self::License => "Microsoft.VisualStudio.Services.Content.License",
            Self::Repository => "Microsoft.VisualStudio.Services.Links.Source",
            Self::Signature => "Microsoft.VisualStudio.Services.VsixSignature",
        };

        write!(f, "{v}")
    }
}
