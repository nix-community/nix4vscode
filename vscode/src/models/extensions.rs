use vscode_derive::api;

// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensions/common/extensions.ts
#[api(Default)]
#[derive(PartialEq, Eq)]
pub enum TargetPlatform {
    Win32X64,
    Win32Ia32,
    Win32Arm64,

    #[default]
    LinuxX64,
    LinuxArm64,
    LinuxArmhf,

    AlpineX64,
    AlpineArm64,

    DarwinX64,
    DarwinArm64,

    Web,

    Universal,
    Unknown,
    Undefined,
}

impl TargetPlatform {
    pub fn is_target_platform_compatible(
        extension_target_platform: TargetPlatform,
        all_target_platforms: &[TargetPlatform],
        product_target_platform: TargetPlatform,
    ) -> bool {
        // Not compatible when extension is not a web extension in web target platform
        if Self::is_not_web_extension_in_web_target_platform(
            all_target_platforms,
            product_target_platform,
        ) {
            return false;
        }

        // Compatible when extension target platform is not defined
        if extension_target_platform == TargetPlatform::Undefined {
            return true;
        }

        // Compatible when extension target platform is universal
        if extension_target_platform == TargetPlatform::Universal {
            return true;
        }

        // Not compatible when extension target platform is unknown
        if extension_target_platform == TargetPlatform::Unknown {
            return false;
        }

        // Compatible when extension and product target platforms matches
        if extension_target_platform == product_target_platform {
            return true;
        }

        false
    }

    pub fn is_not_web_extension_in_web_target_platform(
        all_target_platforms: &[TargetPlatform],
        product_target_platform: TargetPlatform,
    ) -> bool {
        // Not a web extension in web target platform
        product_target_platform == TargetPlatform::Web
            && !all_target_platforms.contains(&TargetPlatform::Web)
    }
}

impl From<&str> for TargetPlatform {
    fn from(value: &str) -> Self {
        match value {
            "win32_x64" | "win32-x64" => Self::Win32X64,
            "win32_ia32" | "win32-ia32" => Self::Win32Ia32,
            "win32_arm64" | "win32-arm64" => Self::Win32Arm64,
            "linux_x64" | "linux-x64" => Self::LinuxX64,
            "linux_arm64" | "linux-arm64" => Self::LinuxArm64,
            "linux_armhf" | "linux-armhf" => Self::LinuxArmhf,
            "alpine_x64" | "alpine-x64" => Self::AlpineX64,
            "alpine_arm64" | "alpine-arm64" => Self::AlpineArm64,
            "darwin_x64" | "darwin-x64" => Self::DarwinX64,
            "darwin_arm64" | "darwin-arm64" => Self::DarwinArm64,
            "web" => Self::Web,
            "universal" => Self::Universal,
            "unknown" => Self::Unknown,
            "undefined" => Self::Undefined,
            _ => Self::Undefined,
        }
    }
}

#[api(Default)]
pub struct MetaDataItem {
    pub name: String,
    pub count: u64,
}

#[api(Default)]
pub struct ResultMetaData {
    pub metadata_type: String,
    pub metadata_items: Vec<MetaDataItem>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert!(matches!("universal".into(), TargetPlatform::Universal));
    }
}
