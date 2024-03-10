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
