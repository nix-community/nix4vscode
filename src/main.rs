#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(lazy_cell)]
#![feature(async_closure)]

pub mod config;
pub mod data_struct;
pub mod error;
pub mod jinja;
pub mod openvsx_ext;
pub mod request;
pub mod utils;

use anyhow::anyhow;
use data_struct::IRawGalleryExtension;

use lazy_regex::regex;
use semver::Version;
use std::{str::FromStr, sync::Arc};
use tracing::*;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use futures::future::join_all;

use clap::Parser;
use config::Config;

use crate::{
    data_struct::{AssetType, TargetPlatform},
    jinja::{AssetUrlContext, Generator, GeneratorContext, NixContext},
    request::HttpClient,
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

#[derive(Debug)]
struct RequiredVersion {
    ver: Version,
    is_upper: bool,
    is_preview: bool,
    is_any: bool,
}

impl RequiredVersion {
    fn new(version: &str) -> anyhow::Result<Self> {
        if version.trim() == "*" {
            return Ok(Self {
                ver: Version::new(0, 0, 0),
                is_upper: false,
                is_preview: false,
                is_any: true,
            });
        }
        let v = regex!(r#"(\^)?(\d+.\d+.\d+)(-.*)?"#);
        let v = v
            .captures(version)
            .ok_or(anyhow!(format!("bad version: {version}")))?;

        Ok(Self {
            ver: semver::Version::from_str(
                v.get(2)
                    .ok_or(anyhow!(format!("bad version: {version}")))?
                    .as_str(),
            )?,
            is_upper: v.get(1).is_some(),
            is_preview: v.get(3).is_some(),
            is_any: false,
        })
    }

    fn is_matched(&self, v: &semver::Version) -> bool {
        if self.is_any {
            return true;
        }
        if self.is_upper {
            return v >= &self.ver;
        }

        v == &self.ver
    }
}

async fn get_matched_versoin(
    item: IRawGalleryExtension,
    vscode_ver: Version,
    client: HttpClient,
    config: Arc<Config>,
    generator: Generator<'_>,
) -> Vec<NixContext> {
    let mx = item
        .versions
        .iter()
        .filter(|v| match v.get_engine() {
            Ok(ver) => {
                let Ok(vv) = RequiredVersion::new(&ver) else {
                    debug!("parse {ver} to RequiredVersion failed.");
                    return false;
                };
                if !vv.is_matched(&vscode_ver) {
                    trace!("{ver} doesn't match {vscode_ver}");
                    return false;
                }
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
    let vscode_ver = semver::Version::from_str(&config.vscode_version).unwrap();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_visual_studio_code_compatibility() {
        let v = semver::Version::new(1, 8, 1);

        let vs = [
            ("1.8.1", true),
            ("^0.1.1", true),
            ("^1.2.1", true),
            ("1.81.1-inside", false),
            ("1.0.1-inside", false),
            ("^1.81.1-inside", false),
            ("^1.0.1-inside", true),
            ("1.12.1", false),
            ("^1.12.1", false),
            ("*", true),
        ];

        for (i, k) in vs {
            let a = RequiredVersion::new(i).unwrap();
            assert!(a.is_matched(&v) == k);
        }
    }
}
