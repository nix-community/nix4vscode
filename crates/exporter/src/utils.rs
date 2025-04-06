pub fn init_logger() {
    use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

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

pub fn get_assert_url(
    is_open_vsx: bool,
    publisher: &str,
    name: &str,
    version: &str,
    platform: Option<&str>,
) -> String {
    let platform_suffix = match platform {
        Some(platform) if !platform.is_empty() => {
            format!("targetPlatform={platform}")
        }
        _ => String::default(),
    };
    if !is_open_vsx {
        return format!(
            "https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage?{platform_suffix}"
        );
    }

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
