use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
    LocalizedLanguages,
    WebExtension,
    SponsorLink,
}

impl ToString for PropertyType {
    fn to_string(&self) -> String {
        match self {
            Self::Dependency => "Microsoft.VisualStudio.Code.ExtensionDependencies".into(),
            Self::ExtensionPack => "Microsoft.VisualStudio.Code.ExtensionPack".into(),
            Self::Engine => "Microsoft.VisualStudio.Code.Engine".into(),
            Self::PreRelease => "Microsoft.VisualStudio.Code.PreRelease".into(),
            Self::LocalizedLanguages => "Microsoft.VisualStudio.Code.LocalizedLanguages".into(),
            Self::WebExtension => "Microsoft.VisualStudio.Code.WebExtension".into(),
            Self::SponsorLink => "Microsoft.VisualStudio.Code.SponsorLink".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SortOrder(u8);

impl SortOrder {
    pub const DEFAULT: Self = Self(0);
    pub const ASCENDING: Self = Self(1);
    pub const DESCENDING: Self = Self(2);
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
