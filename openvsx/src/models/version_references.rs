/*
 * Open VSX Registry API
 *
 * This API provides metadata of VS Code extensions in the Open VSX Registry as well as means to publish extensions.
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// VersionReferences : List of version references matching an extension

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionReferences {
    /// Indicates success of the operation (omitted if a more specific result type is returned)
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// Indicates a warning; when this is present, other properties can still be used
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    /// Indicates an error; when this is present, all other properties should be ignored
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Number of skipped entries according to the version references request
    #[serde(rename = "offset")]
    pub offset: i32,
    /// Total number of version references the extension has
    #[serde(rename = "totalSize")]
    pub total_size: i32,
    /// Essential metadata of all available versions, limited to the size specified in the version references request
    #[serde(rename = "versions")]
    pub versions: Vec<crate::models::VersionReference>,
}

impl VersionReferences {
    /// List of version references matching an extension
    pub fn new(
        offset: i32,
        total_size: i32,
        versions: Vec<crate::models::VersionReference>,
    ) -> VersionReferences {
        VersionReferences {
            success: None,
            warning: None,
            error: None,
            offset,
            total_size,
            versions,
        }
    }
}
