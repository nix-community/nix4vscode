use code_api::code::TargetPlatform;
use minijinja::Value;
use nixpkgs_fmt::reformat_string;

pub fn nixfmt(value: &str) -> String {
    reformat_string(value)
}

pub fn to_string(value: Value) -> String {
    format!(r#""{value}""#)
}

pub fn is_universal(target_platform: String) -> bool {
    let target_platform: TargetPlatform = target_platform.as_str().into();
    matches!(target_platform, TargetPlatform::Universal)
}

pub fn is_linux_x86(target_platform: String) -> bool {
    let target_platform: TargetPlatform = target_platform.as_str().into();
    matches!(target_platform, TargetPlatform::LinuxX64)
}

pub fn is_linux_arm(target_platform: String) -> bool {
    let target_platform: TargetPlatform = target_platform.as_str().into();
    matches!(target_platform, TargetPlatform::LinuxArm64)
}
pub fn is_darwin_x86(target_platform: String) -> bool {
    let target_platform: TargetPlatform = target_platform.as_str().into();
    matches!(target_platform, TargetPlatform::DarwinX64)
}

pub fn is_darwin_arm(target_platform: String) -> bool {
    let target_platform: TargetPlatform = target_platform.as_str().into();
    matches!(target_platform, TargetPlatform::DarwinArm64)
}
