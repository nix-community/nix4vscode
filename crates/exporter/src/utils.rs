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
        let platform_suffix = match platform {
            Some(platform) if !platform.is_empty() => {
                format!("targetPlatform={platform}")
            }
            _ => String::default(),
        };

        return format!(
            "https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?{platform_suffix}"
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
