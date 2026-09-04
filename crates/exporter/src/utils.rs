pub fn init_logger() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_writer(std::io::stderr),
        )
        .with(EnvFilter::from_default_env())
        .init();
}

pub fn render_assert_url(
    is_open_vsx: bool,
    publisher: &str,
    name: &str,
    version: &str,
    platform: Option<&str>,
) -> String {
    if !is_open_vsx {
        let query = match platform {
            Some(platform) if !platform.is_empty() => {
                format!("targetPlatform={platform}&redirect=true")
            }
            _ => "redirect=true".to_string(),
        };

        return format!(
            "https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?{query}"
        );
    }

    let platform_suffix = match platform {
        Some(platform) if !platform.is_empty() => {
            format!("@{platform}")
        }
        _ => String::default(),
    };
    let platform_infix = match platform {
        Some(platform) if !platform.is_empty() => {
            format!("/{platform}")
        }
        _ => String::default(),
    };
    let ext_name = format!("{publisher}.{name}");

    format!(
        "https://open-vsx.org/api/{publisher}/{name}{platform_infix}/{version}/file/{ext_name}-{version}{platform_suffix}.vsix"
    )
}

pub fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let mut a_parts = a.split('.');
    let mut b_parts = b.split('.');

    loop {
        match (a_parts.next(), b_parts.next()) {
            (Some(a_part), Some(b_part)) => {
                let a_num = a_part.parse::<u32>().unwrap_or_default();
                let b_num = b_part.parse::<u32>().unwrap_or_default();

                match a_num.cmp(&b_num) {
                    std::cmp::Ordering::Equal => continue,
                    ordering => return ordering,
                }
            }
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (None, None) => return std::cmp::Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_assert_url_vscode_with_platform() {
        let url = render_assert_url(
            false,
            "extensionPublisher",
            "extensionName",
            "1.0.0",
            Some("linux-x64"),
        );
        assert_eq!(
            url,
            "https://extensionPublisher.gallery.vsassets.io/_apis/public/gallery/publisher/extensionPublisher/extension/extensionName/1.0.0/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?targetPlatform=linux-x64&redirect=true"
        );
    }

    #[test]
    fn test_render_assert_url_vscode_universal() {
        let url = render_assert_url(false, "Google", "google-antigravity", "1.0.0", None);
        assert_eq!(
            url,
            "https://Google.gallery.vsassets.io/_apis/public/gallery/publisher/Google/extension/google-antigravity/1.0.0/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?redirect=true"
        );

        let url_empty = render_assert_url(false, "Google", "google-antigravity", "1.0.0", Some(""));
        assert_eq!(
            url_empty,
            "https://Google.gallery.vsassets.io/_apis/public/gallery/publisher/Google/extension/google-antigravity/1.0.0/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?redirect=true"
        );
    }

    #[test]
    fn test_render_assert_url_openvsx() {
        let url = render_assert_url(
            true,
            "extensionPublisher",
            "extensionName",
            "1.0.0",
            Some("linux-x64"),
        );
        assert_eq!(
            url,
            "https://open-vsx.org/api/extensionPublisher/extensionName/linux-x64/1.0.0/file/extensionPublisher.extensionName-1.0.0@linux-x64.vsix"
        );

        let url_no_platform =
            render_assert_url(true, "extensionPublisher", "extensionName", "1.0.0", None);
        assert_eq!(
            url_no_platform,
            "https://open-vsx.org/api/extensionPublisher/extensionName/1.0.0/file/extensionPublisher.extensionName-1.0.0.vsix"
        );
    }
}
