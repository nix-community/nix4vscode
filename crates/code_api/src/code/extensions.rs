// https://github.com/microsoft/vscode/blob/d187d50a482ff80dcf74c35affb09dda1a7cd2fe/src/vs/platform/extensions/common/extensions.ts
use super::TargetPlatform;

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
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert!(matches!("universal".into(), TargetPlatform::Universal));
    }
}
