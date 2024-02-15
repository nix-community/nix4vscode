use super::*;
use std::fmt::Display;
use vscode_derive::api;

use super::IRawGalleryExtensionVersion;

#[api(Default)]
pub struct IRawGalleryQueryResult {
    results: Vec<IRawGalleryExtension>,
}

#[api(Default)]
pub struct IRawGalleryExtensionStatistics {
    statistic_name: String,
    value: f64,
}

#[api(Default)]
pub struct IRawGalleryExtensionPublisher {
    display_name: String,
    publisher_id: String,
    publisher_name: String,
    domain: Option<String>,
    is_domain_verified: bool,
}

#[api(Default)]
pub struct IRawGalleryExtension {
    extension_id: String,
    extension_name: String,
    display_name: String,
    short_description: Option<String>,
    publisher: IRawGalleryExtensionPublisher,
    versions: Vec<IRawGalleryExtensionVersion>,
    statistics: Vec<IRawGalleryExtensionStatistics>,
    tags: Option<String>,
    release_date: String,
    published_date: String,
    last_updated: String,
    categories: Option<Vec<String>>,
    flags: String,
}

impl Display for IRawGalleryExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.publisher.publisher_name, self.extension_name
        )
    }
}

#[api(Default)]
pub struct IRawGalleryExtensionsResult {
    extensions: Vec<IRawGalleryExtension>,
    result_metadata: Vec<ResultMetaData>,
}

#[api(Default)]
pub struct IGalleryExtension {
    name: String,
    // identifier: IGalleryExtensionIdentifier,
    version: String,
    display_name: String,
    publisher_id: String,
    publisher: String,
    publisher_display_name: String,
    // publisherDomain?: { link: String; verified: boolean },
    publisher_sponsor_link: Option<String>,
    description: String,
    install_count: usize,
    rating: usize,
    rating_count: usize,
    categories: Vec<String>,
    tags: Vec<String>,
    release_date: usize,
    last_updated: usize,
    preview: bool,
    has_pre_release_version: bool,
    has_release_version: bool,
    is_signed: bool,
    all_target_platforms: Vec<TargetPlatform>,
    assets: IGalleryExtensionAssets,
    properties: IGalleryExtensionProperties,
    // telemetryData?: any,
    // queryContext?: IStringDictionary<any>,
    support_link: Option<String>,
}

impl IGalleryExtension {
    pub fn create(
        raw_gallery_extension: IRawGalleryExtension,
        criteria: IExtensionCriteria,
    ) -> Self {
        todo!()
    }
}

#[api(Default)]
pub struct IGalleryExtensionAssets {
    manifest: Option<IGalleryExtensionAsset>,
    readme: Option<IGalleryExtensionAsset>,
    changelog: Option<IGalleryExtensionAsset>,
    license: Option<IGalleryExtensionAsset>,
    repository: Option<IGalleryExtensionAsset>,
    download: IGalleryExtensionAsset,
    icon: Option<IGalleryExtensionAsset>,
    signature: Option<IGalleryExtensionAsset>,
    // coreTranslations: [string, IGalleryExtensionAsset][],
}

#[api(Default)]
pub struct IGalleryExtensionProperties {
    dependencies: Vec<String>,
    extension_pack: Vec<String>,
    engine: Option<String>,
    localized_languages: Option<String>,
    target_platform: TargetPlatform,
    is_pre_release_version: bool,
}
