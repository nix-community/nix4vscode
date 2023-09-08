#![allow(dead_code)]
#![allow(unused_variables)]

use log::*;
use std::str::FromStr;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{fmt, util::SubscriberInitExt};

use clap::Parser;
use config::{Config, Extension};
use data::{NixContext, PackageJson};
use request::{FilterType, ICriterium, IQueryState, Query, RequestFlags};
use tokio::fs;

use crate::data::AssetType;

pub mod config;
pub mod data;
pub mod jinja;
pub mod request;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("NIX4VSCODE"))
        .init();

    let config: Config =
        toml::from_str(fs::read_to_string(args.file).await.unwrap().as_str()).unwrap();
    let mut query_state = IQueryState {
        criteria: vec![
            ICriterium {
                filter_type: FilterType::TARGET,
                value: "Microsoft.VisualStudio.Code".into(),
            },
            ICriterium {
                filter_type: FilterType::EXCLUDE_WITH_FLAGS,
                value: "4096".into(),
            },
        ],
        ..Default::default()
    };

    config.extensions.iter().for_each(|item| {
        query_state.criteria.push(ICriterium {
            filter_type: FilterType::EXTENSION_NAME,
            value: format!("{}.{}", item.publisher_name, item.extension_name),
        });
    });

    let query = Query {
        filters: vec![query_state],
        asset_types: Default::default(),
        flags: (RequestFlags::IncludeVersions
            | RequestFlags::include_asset_uri
            | RequestFlags::include_files)
            .bits(),
    };
    let query = serde_json::to_string(&query).unwrap();
    debug!("{query}");

    let client = reqwest::Client::builder().gzip(true).build().unwrap();
    debug!("request");
    let res = client
        .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
        .header(
            "Accept",
            "application/json; charset=utf-8; api-version=7.2-preview.1",
        )
        .header("CONTENT-TYPE", "application/json")
        .body(query)
        .send()
        .await
        .unwrap();

    let query = res.text().await.unwrap();

    let vscode_ver = semver::Version::from_str(&config.vscode_version).unwrap();
    let mut res: Vec<NixContext> = Default::default();

    let obj: data::IRawGalleryQueryResult = serde_json::from_str(query.as_str()).unwrap();
    for item in obj.results[0].extensions.iter().filter(|item| {
        config.extensions.contains(&Extension {
            publisher_name: item.publisher.publisher_name.clone(),
            extension_name: item.extension_name.clone(),
        })
    }) {
        for version in &item.versions {
            // Get From [version]
            let file = version.get_file(AssetType::Manifest).unwrap();
            let package = reqwest::get(file.source.clone())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            trace!("get {}", file.source);
            let package: PackageJson = serde_json::from_str(&package).unwrap();
            let required_ver = semver::VersionReq::from_str(&package.engines.vscode).unwrap();
            info!("get version:{}", package.engines.vscode);
            if required_ver.matches(&vscode_ver) {
                res.push(NixContext {
                    extension_name: item.extension_name.clone(),
                    publisher_name: item.publisher.publisher_name.clone(),
                    extension_version: version.version.clone(),
                    asset_url: version.get_file(AssetType::Vsix).unwrap().source.clone(),
                    sha256: Default::default(),
                });
                break;
            }
        }
    }

    for item in &mut res {
        let sha256 = tokio::process::Command::new("nix-prefetch-url")
            .arg(item.asset_url.clone())
            .output()
            .await
            .unwrap()
            .stdout;
        let sha256 = String::from_utf8(sha256).unwrap();
        item.sha256 = sha256;
    }
    info!("{res:?}");

    let mut generator = minijinja::Environment::new();
    generator
        .add_template(
            "nix_expression",
            include_str!("./jinja/nix_expression.nix.j2"),
        )
        .unwrap();
    generator.add_global("NixContexts", minijinja::Value::from_serializable(&res));
    let res = generator
        .get_template("nix_expression")
        .unwrap()
        .render(minijinja::Value::default())
        .unwrap();
    println!("{res}");
}
