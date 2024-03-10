#![allow(dead_code)]
#![allow(unused_variables)]

pub mod code;
pub mod config;
pub mod error;
pub mod jinja;
pub mod openvsx_ext;
pub mod utils;
mod version;

use code::{HttpClient, IRawGalleryExtension};

use semver::Version;
use std::{str::FromStr, sync::Arc};
use tracing::*;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use futures::future::join_all;

use clap::Parser;
use config::Config;

use crate::{
    code::{AssetType, TargetPlatform},
    jinja::{AssetUrlContext, Generator, GeneratorContext, NixContext},
    version::is_version_valid,
};

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    file: String,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(long, hide = true)]
    export: bool,
    #[arg(long, hide = true)]
    dump: bool,
    #[arg(long)]
    openvsx: bool,
}

async fn get_matched_versoin(
    item: IRawGalleryExtension,
    vscode_ver: String,
    client: HttpClient,
    config: Arc<Config>,
    generator: Generator<'_>,
) -> Vec<NixContext> {
    let mx = item
        .versions
        .iter()
        .filter(|v| match v.get_engine() {
            Ok(ver) => {
                if !is_version_valid(&vscode_ver, &ver) {
                    trace!("{ver} doesn't match {vscode_ver:?}");
                    return false;
                }
                trace!("{} - {}", v.version, ver);
                true
            }
            Err(_) => {
                debug!(
                    "Cannot get engine version for {}.{} {}",
                    item.publisher.publisher_name, item.extension_name, v
                );
                trace!("{v:#?}");
                true
            }
        })
        .filter_map(|item| Version::from_str(&item.version).ok())
        .max_by(|a, b| a.cmp(b));

    let mx = mx.map(|item| item.to_string());
    trace!(?mx);

    let mut res = vec![];
    for version in &item.versions {
        if let Some(mx) = mx.as_ref() {
            if mx != &version.version {
                continue;
            }
        }
        trace!("{:?}", version.version);
        if let Some(ref v) = version.target_platform {
            let t: TargetPlatform = v.as_str().into();
            if !matches!(
                t,
                TargetPlatform::LinuxX64
                    | TargetPlatform::LinuxArm64
                    | TargetPlatform::Universal
                    | TargetPlatform::Web
                    | TargetPlatform::DarwinX64
                    | TargetPlatform::DarwinArm64
            ) {
                continue;
            }
        }
        let (has_asset_url, asset_url) = match config
            .get_asset_url(&item.publisher.publisher_name, &item.extension_name)
        {
            Some(url) => {
                let url = generator.render_asset_url(
                    &url,
                    &AssetUrlContext::new(
                        config
                            .get_system_ctx(&item.publisher.publisher_name, &item.extension_name)
                            .unwrap_or_default(),
                        version.version.clone(),
                    ),
                );
                (true, url)
            }
            None => (
                false,
                version.get_file(AssetType::Vsix).unwrap().source.clone(),
            ),
        };
        debug!(
            "{}-{}-{:?}",
            item.publisher.publisher_name, item.extension_name, asset_url
        );

        let sha256 = match utils::get_sha256(&asset_url).await {
            Ok(sha256) => sha256,
            Err(err) => {
                error!("get sha256 failed: {err}");
                continue;
            }
        };

        let target_platform = match version.target_platform {
            Some(ref t) => vec![t.as_str().into()],
            None => {
                client
                    .get_extension_target_platform(
                        item.publisher.publisher_name.clone(),
                        item.extension_name.clone(),
                    )
                    .await
            }
        };
        trace!(?target_platform);

        let a = target_platform
            .into_iter()
            .filter(|item| {
                matches!(
                    *item,
                    TargetPlatform::LinuxX64
                        | TargetPlatform::LinuxArm64
                        | TargetPlatform::Universal
                        | TargetPlatform::Web
                        | TargetPlatform::DarwinX64
                        | TargetPlatform::DarwinArm64
                )
            })
            .map(|target_platform| NixContext {
                extension_name: item.extension_name.to_lowercase(),
                publisher_name: item.publisher.publisher_name.to_lowercase(),
                extension_version: version.version.clone(),
                asset_url: if has_asset_url {
                    Some(asset_url.clone())
                } else {
                    None
                },
                sha256: sha256.clone(),
                target_platform,
            });

        res.extend(a);
    }

    trace!(?res);

    res
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger();

    let config = Arc::new(Config::new(
        tokio::fs::read_to_string(&args.file).await?.as_str(),
    )?);
    let client = HttpClient::new().unwrap();
    debug!("request: {config:?}");
    let vscode_ver = config.vscode_version.to_string();
    let mut generator = Generator::new();

    let res: Vec<_> = {
        let obj = client
            .get_extension_response(&config.extensions)
            .await
            .unwrap();

        let futures: Vec<_> = obj
            .results
            .into_iter()
            .flat_map(|item| item.extensions.into_iter())
            .filter(|item| {
                match config.contains(&item.publisher.publisher_name, &item.extension_name) {
                    true => true,
                    false => {
                        debug!(
                            "extensions be filtered {}.{}",
                            item.publisher.publisher_name, item.extension_name
                        );
                        false
                    }
                }
            })
            .map(|item| {
                trace!("aa");
                let vscode_ver = vscode_ver.clone();
                let client = client.clone();
                let config = Arc::clone(&config);
                let generator = generator.clone();
                get_matched_versoin(item, vscode_ver, client, config, generator)
            })
            .collect();

        join_all(futures).await.into_iter().flatten().collect()
    };

    debug!("{res:#?}");
    if args.export {
        let res = serde_json::to_string(&res).unwrap();
        match args.output {
            Some(filepath) => tokio::fs::write(filepath, res).await.unwrap(),
            None => println!("{res}",),
        }
        return Ok(());
    }

    let res = generator.render(&GeneratorContext {
        extensions: res,
        config: config.clone(),
    })?;

    match args.output {
        Some(filepath) => tokio::fs::write(filepath, res).await.unwrap(),
        None => println!("{res}",),
    }

    Ok(())
}

fn init_logger() {
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or("WARN".into())
        .to_lowercase();

    let env_filter = EnvFilter::builder()
        .parse(format!("RUST_LOG=OFF,nix4vscode={}", log_level))
        .unwrap();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_writer(std::io::stderr),
        )
        .with(env_filter)
        .init();
}
