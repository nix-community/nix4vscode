use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct RequestFlags: u32 {
        ///
        /// None is used to retrieve only the basic extension details.
        ///
        const None = 0x0;

        ///
        /// IncludeVersions will return version information for extensions returned
        ///
        const IncludeVersions = 0x1;

        ///
        /// IncludeFiles will return information about which files were found
        /// within the extension that were stored independent of the manifest.
        /// When asking for files, versions will be included as well since files
        /// are returned as a property of the versions.
        /// These files can be retrieved using the path to the file without
        /// requiring the entire manifest be downloaded.
        ///
        const include_files = 0x2;

        ///
        ///Include the Categories and Tags that were added to the extension definition.
        ///
        const include_category_and_tags = 0x4;

        ///
        ///Include the details about which accounts the extension has been shared
        ///with if the extension is a private extension.
        ///
        const include_shared_accounts = 0x8;

        ///
        ///Include properties associated with versions of the extension
        ///
        const include_version_properties = 0x10;

        ///
        ///Excluding non-validated extensions will remove any extension versions that
        ///either are in the process of being validated or have failed validation.
        ///
        const exclude_non_validated = 0x20;

        ///
        ///Include the set of installation targets the extension has requested.
        ///
        const include_installation_targets = 0x40;

        ///
        ///Include the base uri for assets of this extension
        ///
        const include_asset_uri = 0x80;

        ///
        ///Include the statistics associated with this extension
        ///
        const include_statistics = 0x100;

        ///
        ///When retrieving versions from a query, only include the latest
        ///version of the extensions that matched. This is useful when the
        ///caller doesn't need all the published versions. It will save a
        ///significant size in the returned payload.
        ///
        const include_latest_version_only = 0x200;

        ///
        ///This flag switches the asset uri to use GetAssetByName instead of CDN
        ///When this is used, values of base asset uri and base asset uri fallback are switched
        ///When this is used, source of asset files are pointed to Gallery service always even if CDN is available
        ///
        const unpublished = 0x1000;

        ///
        ///Include the details if an extension is in conflict list or not
        ///
        const includeNameConflictInfo = 0x8000;
    }
}

impl Default for RequestFlags {
    fn default() -> Self {
        RequestFlags::IncludeVersions
            | RequestFlags::include_asset_uri
            | RequestFlags::include_files
    }
}
