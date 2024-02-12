use vscode_derive::api;

#[api]
pub enum FilterType {
    Tag = 1,
    ExtensionId = 4,
    Category = 5,
    ExtensionName = 7,
    Target = 8,
    Featured = 9,
    SearchText = 10,
    ExcludeWithFlags = 12,
}

impl Default for FilterType {
    fn default() -> Self {
        Self::Tag
    }
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

#[api(Default)]
pub enum SortOrder {
    #[default]
    Default = 0,
    Ascending = 1,
    Descending = 2,
}

#[api(Default)]
pub enum SortBy {
    #[default]
    NoneOrRelevance = 0,
    LastUpdatedDate = 1,
    Title = 2,
    PublisherName = 3,
    InstallCount = 4,
    PublishedDate = 10,
    AverageRating = 6,
    WeightedRating = 12,
}
