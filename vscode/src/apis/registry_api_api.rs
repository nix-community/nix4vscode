use crate::models::{IExtensionInfo, IGalleryExtension, TargetPlatform};

use super::Configuration;

impl Configuration {
    pub async fn get_compatible_extension(
        extension: IGalleryExtension,
        include_pre_release: bool,
        target_platform: TargetPlatform,
    ) -> Option<IGalleryExtension> {
        todo!()
    }

    pub async fn get_extensions(extension_infos: Vec<IExtensionInfo>) -> Vec<IGalleryExtension> {
        todo!()
    }
}
