use vscode_derive::api;

use super::TargetPlatform;

#[api(Default)]
pub struct IExtensionInfo {
    id: String,
    uuid: Option<String>,
    version: Option<String>,
    pre_release: Option<bool>,
    has_pre_release: Option<bool>,
}

#[api(Default)]
pub struct IExtensionQueryOptions {
    target_platform: Option<TargetPlatform>,
    compatible: Option<bool>,
    query_all_versions: Option<bool>,
    source: Option<String>,
}
