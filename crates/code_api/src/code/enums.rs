use std::fmt::Display;

use derive::api;

#[api(nodefault)]
pub struct FilterType(u8);

impl Default for FilterType {
    fn default() -> Self {
        Self::TAG
    }
}

impl FilterType {
    pub const TAG: Self = Self(1);
    pub const EXTENSION_ID: Self = Self(4);
    pub const CATEGORY: Self = Self(5);
    pub const EXTENSION_NAME: Self = Self(7);
    pub const TARGET: Self = Self(8);
    pub const FEATURED: Self = Self(9);
    pub const SEARCH_TEXT: Self = Self(10);
    pub const EXCLUDE_WITH_FLAGS: Self = Self(12);
}

pub enum PropertyType {
    Dependency,
    ExtensionPack,
    Engine,
    PreRelease,
    EnabledApiProposals,
    LocalizedLanguages,
    WebExtension,
    SponsorLink,
    SupportLink,
    ExecutesCode,
}

impl PropertyType {
    pub const DEPENDENCY: &str = "Microsoft.VisualStudio.Code.ExtensionDependencies";
    pub const EXTENSION_PACK: &str = "Microsoft.VisualStudio.Code.ExtensionPack";
    pub const ENGINE: &str = "Microsoft.VisualStudio.Code.Engine";
    pub const PRE_RELEASE: &str = "Microsoft.VisualStudio.Code.PreRelease";
    pub const ENABLED_API_PROPOSALS: &str = "Microsoft.VisualStudio.Code.EnabledApiProposals";
    pub const LOCALIZED_LANGUAGES: &str = "Microsoft.VisualStudio.Code.LocalizedLanguages";
    pub const WEB_EXTENSION: &str = "Microsoft.VisualStudio.Code.WebExtension";
    pub const SPONSOR_LINK: &str = "Microsoft.VisualStudio.Code.SponsorLink";
    pub const SUPPORT_LINK: &str = "Microsoft.VisualStudio.Services.Links.Support";
    pub const EXECUTES_CODE: &str = "Microsoft.VisualStudio.Code.ExecutesCode";
}

impl Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: &str = self.into();
        write!(f, "{}", v)
    }
}

impl From<&PropertyType> for &str {
    fn from(value: &PropertyType) -> Self {
        match value {
            PropertyType::Dependency => "Microsoft.VisualStudio.Code.ExtensionDependencies",
            PropertyType::ExtensionPack => "Microsoft.VisualStudio.Code.ExtensionPack",
            PropertyType::Engine => "Microsoft.VisualStudio.Code.Engine",
            PropertyType::PreRelease => "Microsoft.VisualStudio.Code.PreRelease",
            PropertyType::EnabledApiProposals => "Microsoft.VisualStudio.Code.EnabledApiProposals",
            PropertyType::LocalizedLanguages => "Microsoft.VisualStudio.Code.LocalizedLanguages",
            PropertyType::WebExtension => "Microsoft.VisualStudio.Code.WebExtension",
            PropertyType::SponsorLink => "Microsoft.VisualStudio.Code.SponsorLink",
            PropertyType::SupportLink => "Microsoft.VisualStudio.Services.Links.Support",
            PropertyType::ExecutesCode => "Microsoft.VisualStudio.Code.ExecutesCode",
        }
    }
}

#[api]
pub struct SortOrder(u8);

impl SortOrder {
    pub const DEFAULT: Self = Self(0);
    pub const ASCENDING: Self = Self(1);
    pub const DESCENDING: Self = Self(2);
}

#[api]
pub struct SortBy(u8);

impl SortBy {
    pub const NONE_OR_RELEVANCE: Self = Self(0);
    pub const LAST_UPDATED_DATE: Self = Self(1);
    pub const TITLE: Self = Self(2);
    pub const PUBLISHER_NAME: Self = Self(3);
    pub const INSTALL_COUNT: Self = Self(4);
    pub const PUBLISHED_DATE: Self = Self(10);
    pub const AVERAGE_RATING: Self = Self(6);
    pub const WEIGHTED_RATING: Self = Self(12);
}
