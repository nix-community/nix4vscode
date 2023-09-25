use serde::{Deserialize, Serialize};

// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensions/common/extensions.ts
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum TargetPlatform {
    Win32X64,
    Win32Ia32,
    Win32Arm64,

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetaDataItem {
    name: String,
    count: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultMetaData {
    metadata_type: String,
    metadata_items: Vec<MetaDataItem>,
}
