#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(lazy_cell)]
#![feature(async_closure)]

pub mod config;
pub mod data_struct;
pub mod dump;
pub mod error;
pub mod jinja;
pub mod request;
pub mod utils;

use data_struct::IRawGalleryExtension;
use log::*;
use semver::Version;
use std::{str::FromStr, sync::Arc};
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use futures::future::join_all;

use clap::Parser;
use config::Config;

use crate::{
    data_struct::AssetType,
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
}

async fn get_matched_versoin(
    item: IRawGalleryExtension,
    vscode_ver: Version,
    client: HttpClient,
    config: Arc<Config>,
    generator: Generator<'_>,
) -> Option<NixContext> {
    for version in &item.versions {
        match version.get_engine() {
            Ok(ver) => {
                if ver.matches(&vscode_ver) {
                    continue;
                }
            }
            Err(_) => {
                warn!(
                    "Cannot get engine version for {}.{} {}",
                    item.publisher.publisher_name, item.extension_name, version
                );
                trace!("{version:#?}");
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
                error!("{err}");
                return None;
            }
        };

        return Some(NixContext {
            extension_name: item.extension_name.clone(),
            publisher_name: item.publisher.publisher_name.clone(),
            extension_version: version.version.clone(),
            asset_url: if has_asset_url {
                Some(asset_url.clone())
            } else {
                None
            },
            sha256,
            target_platform: client
                .get_extension_target_platform(item.publisher.publisher_name, item.extension_name)
                .await,
        });
    }
    None
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger();

    let config = Arc::new(Config::new(&args.file).await?);
    let client = HttpClient::new().unwrap();
    debug!("request: {config:?}");
    let vscode_ver = semver::Version::from_str(&config.vscode_version).unwrap();
    let mut generator = Generator::new();

    if args.dump {
        let res = dump::dump(&client, &vscode_ver, &config, &generator).await;
        debug!("find dump of vscode marketplace: \n{res:#?}");
        let res = serde_json::to_string(&res).unwrap();
        match args.output {
            Some(filepath) => tokio::fs::write(filepath, res).await.unwrap(),
            None => println!("{res}",),
        }
        return Ok(());
    }

    let obj = client
        .get_extension_response(&config.extensions)
        .await
        .unwrap();
    trace!("{obj:#?}");

    let futures: Vec<_> = obj
        .results
        .into_iter()
        .flat_map(|item| item.extensions.into_iter())
        .filter(|item| match config.extensions.is_empty() {
            true => true,
            false => config.contains(&item.publisher.publisher_name, &item.extension_name),
        })
        .map(|item| {
            let vscode_ver = vscode_ver.clone();
            let client = client.clone();
            let config = Arc::clone(&config);
            let generator = generator.clone();
            get_matched_versoin(item, vscode_ver, client, config, generator)
        })
        .collect();

    let res: Vec<_> = join_all(futures).await.into_iter().flatten().collect();
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
        .unwrap_or("INFO".into())
        .to_lowercase();

    let env_filter = EnvFilter::builder()
        .parse(format!("RUST_LOG=OFF,nix4vscode={}", log_level))
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer().with_file(true).with_line_number(true))
        .with(env_filter)
        .init();
}
