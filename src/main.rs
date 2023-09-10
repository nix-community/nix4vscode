#![allow(dead_code)]
#![allow(unused_variables)]

use log::*;
use std::{str::FromStr, sync::Arc};
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

use futures::future::join_all;

use clap::Parser;
use config::Config;
use data::{NixContext, PackageJson};
use request::Query;

use crate::{
    data::AssetType,
    jinja::{ExtensionContext, Generator, GeneratorContext},
};

pub mod config;
pub mod data;
pub mod jinja;
pub mod request;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger();

    let config = Arc::new(Config::new(&args.file).await?);
    let client = reqwest::Client::builder().gzip(true).build()?;
    debug!("request: {config:?}");
    let obj = Query::new(&config).get_response(&client).await?;
    let vscode_ver = semver::Version::from_str(&config.vscode_version).unwrap();
    let mut generator = Generator::default();

    let futures: Vec<_> = obj
        .results
        .into_iter()
        .flat_map(|item| item.extensions.into_iter())
        .filter(|item| config.contains(&item.publisher.publisher_name, &item.extension_name))
        .map(|item| {
            let vscode_ver = vscode_ver.clone();
            let client = client.clone();
            let config = Arc::clone(&config);
            let generator = generator.clone();
            async move {
                for version in &item.versions {
                    // Get From [version]
                    let file = version.get_file(AssetType::Manifest).unwrap();
                    let req = client.get(file.source.clone()).build().unwrap();
                    let package = client
                        .execute(req)
                        .await
                        .unwrap()
                        .json::<PackageJson>()
                        .await
                        .unwrap();
                    trace!("get {} - {}", item.extension_name, file.source);
                    let required_ver =
                        semver::VersionReq::from_str(&package.engines.vscode).unwrap();
                    info!(
                        "get {}.{} rquired vscode version:{}",
                        item.publisher.publisher_name, item.extension_name, package.engines.vscode
                    );
                    if required_ver.matches(&vscode_ver) {
                        let (has_asset_url, asset_url) = match config
                            .get_asset_url(&item.publisher.publisher_name, &item.extension_name)
                        {
                            Some(url) => {
                                let url = generator.render_asset_url(
                                    &url,
                                    &ExtensionContext::new(version.version.clone()),
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

                        let sha256 = tokio::process::Command::new("nix-prefetch-url")
                            .arg(asset_url.clone())
                            .output()
                            .await
                            .unwrap()
                            .stdout;
                        let sha256 = String::from_utf8(sha256).unwrap().trim().to_owned();
                        return Some(NixContext {
                            extension_name: item.extension_name.clone(),
                            publisher_name: item.publisher.publisher_name.clone(),
                            extension_version: version.version.clone(),
                            asset_url: if has_asset_url { Some(asset_url) } else { None },
                            sha256,
                        });
                    }
                }
                None
            }
        })
        .collect();

    let res: (Vec<_>, Vec<_>) = join_all(futures)
        .await
        .into_iter()
        .flatten()
        .partition(|item| item.asset_url.is_some());
    debug!("{res:?}");

    let res = generator.render(&GeneratorContext {
        nixs: res.1,
        autogen_warning: config.autogen_warning.clone(),
        reassets: res.0,
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
        .with(fmt::layer())
        .with(env_filter)
        .init();
}
