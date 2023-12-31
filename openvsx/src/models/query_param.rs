/*
 * Open VSX Registry API
 *
 * This API provides metadata of VS Code extensions in the Open VSX Registry as well as means to publish extensions.
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// QueryParam : Parameters of the metadata query

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryParam {
    /// Name of a namespace
    #[serde(rename = "namespaceName", skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// Name of an extension
    #[serde(rename = "extensionName", skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
    /// Version of an extension
    #[serde(rename = "extensionVersion", skip_serializing_if = "Option::is_none")]
    pub extension_version: Option<String>,
    /// Identifier in the form {namespace}.{extension}
    #[serde(rename = "extensionId", skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    /// Universally unique identifier of an extension
    #[serde(rename = "extensionUuid", skip_serializing_if = "Option::is_none")]
    pub extension_uuid: Option<String>,
    /// Universally unique identifier of a namespace
    #[serde(rename = "namespaceUuid", skip_serializing_if = "Option::is_none")]
    pub namespace_uuid: Option<String>,
    /// Whether to include all versions of an extension, ignored if extensionVersion is specified
    #[serde(rename = "includeAllVersions", skip_serializing_if = "Option::is_none")]
    pub include_all_versions: Option<bool>,
    /// Name of the target platform
    #[serde(rename = "targetPlatform", skip_serializing_if = "Option::is_none")]
    pub target_platform: Option<TargetPlatform>,
    /// Maximal number of entries to return
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Number of entries to skip (usually a multiple of the page size)
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl QueryParam {
    /// Parameters of the metadata query
    pub fn new() -> QueryParam {
        QueryParam {
            namespace_name: None,
            extension_name: None,
            extension_version: None,
            extension_id: None,
            extension_uuid: None,
            namespace_uuid: None,
            include_all_versions: None,
            target_platform: None,
            size: None,
            offset: None,
        }
    }
}

/// Name of the target platform
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetPlatform {
    #[serde(rename = "win32-x64")]
    Win32X64,
    #[serde(rename = "win32-ia32")]
    Win32Ia32,
    #[serde(rename = "win32-arm64")]
    Win32Arm64,
    #[serde(rename = "linux-x64")]
    LinuxX64,
    #[serde(rename = "linux-arm64")]
    LinuxArm64,
    #[serde(rename = "linux-armhf")]
    LinuxArmhf,
    #[serde(rename = "alpine-x64")]
    AlpineX64,
    #[serde(rename = "alpine-arm64")]
    AlpineArm64,
    #[serde(rename = "darwin-x64")]
    DarwinX64,
    #[serde(rename = "darwin-arm64")]
    DarwinArm64,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "universal")]
    Universal,
}

impl Default for TargetPlatform {
    fn default() -> TargetPlatform {
        Self::Win32X64
    }
}
