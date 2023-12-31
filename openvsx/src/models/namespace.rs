/*
 * Open VSX Registry API
 *
 * This API provides metadata of VS Code extensions in the Open VSX Registry as well as means to publish extensions.
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// Namespace : Describes the namespace to create

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Namespace {
    /// Indicates success of the operation (omitted if a more specific result type is returned)
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// Indicates a warning; when this is present, other properties can still be used
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    /// Indicates an error; when this is present, all other properties should be ignored
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Name of the namespace
    #[serde(rename = "name")]
    pub name: String,
    /// Map of extension names to their metadata URLs (not required for creating)
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<::std::collections::HashMap<String, String>>,
    /// Indicates whether the namespace has an owner (not required for creating)
    #[serde(rename = "verified")]
    pub verified: bool,
    /// Access level of the namespace. Deprecated: namespaces are now always restricted
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,
}

impl Namespace {
    /// Describes the namespace to create
    pub fn new(name: String, verified: bool) -> Namespace {
        Namespace {
            success: None,
            warning: None,
            error: None,
            name,
            extensions: None,
            verified,
            access: None,
        }
    }
}

/// Access level of the namespace. Deprecated: namespaces are now always restricted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Access {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "restricted")]
    Restricted,
}

impl Default for Access {
    fn default() -> Access {
        Self::Public
    }
}
