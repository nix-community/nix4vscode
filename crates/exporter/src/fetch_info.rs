use std::{collections::HashMap, pin::pin};

use crate::{
    codellab::{TagName, fetch_codelldb},
    models::Marketplace,
};
use code_api::code::{ApiEndpoint, HttpClient, SortBy};
use diesel::prelude::*;
use futures::StreamExt;
use itertools::Itertools;
use octocrab::models::repos::Release;

use crate::schema::marketplace;
use tracing::*;

pub async fn fetch_marketplace(
    conn: &mut SqliteConnection,
    endpoint: ApiEndpoint,
) -> anyhow::Result<()> {
    let client = HttpClient::new(endpoint).unwrap();
    let mut iter = pin!(client.get_extension_response(
        vec![],
        code_api::code::IQueryState {
            page_size: u64::MAX,
            sort_by: SortBy::PUBLISHED_DATE,
            sort_order: code_api::code::SortOrder::DESCENDING,
            ..Default::default()
        }
    ));
    let codelldb = fetch_codelldb().await.unwrap_or_default();
    debug!("codelldb: len = {}", codelldb.len());
    let mut extension_count = 0usize;
    let mut all_count = 0usize;
    while let Some(item) = iter.next().await {
        let item = match item {
            Ok(item) => item,
            Err(err) => {
                error!(?err);
                continue;
            }
        };

        if item.extensions.is_empty() {
            break;
        }

        let values: Vec<_> = item
            .extensions
            .iter()
            .flat_map(|item| {
                extension_count += 1;
                all_count += item.versions.len();
                item.versions.iter().filter_map(|v| {
                    let Ok(engne) = v.get_engine() else {
                        return None;
                    };
                    let platform = v.target_platform.clone().unwrap_or("universal".to_string());
                    Some(Marketplace {
                        name: item.extension_name.clone(),
                        publisher: item.publisher.publisher_name.clone(),
                        version: v.version.clone(),
                        engine: engne,
                        platform,
                        hash: None,
                        is_prerelease: v.is_pre_release_version(),
                        url: None,
                    })
                })
            })
            .flat_map(|item| {
                if item.publisher == "vadimcn" && item.name == "vscode-lldb" {
                    drain_codelldb(&codelldb, item)
                } else {
                    vec![item]
                }
            })
            .collect();

        if let Err(err) = diesel::insert_or_ignore_into(marketplace::table)
            .values(&values)
            .execute(conn)
        {
            error!(?err);
        }

        info!("[{extension_count}] - [{all_count}]");
    }

    Ok(())
}

fn drain_codelldb(codelldb: &HashMap<TagName, Release>, item: Marketplace) -> Vec<Marketplace> {
    let Some(release) = codelldb.get(&TagName(format!("v{}", item.version))) else {
        return vec![item];
    };

    let x = release
        .assets
        .iter()
        .filter_map(|asset| {
            let value = item.clone();
            if is_darwin_aarch64(&asset.name) {
                Some(Marketplace {
                    platform: "darwin-arm64".into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else if is_darwin_x86_64(&asset.name) {
                Some(Marketplace {
                    platform: "darwin-x64".into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else if is_linux_aarch64(&asset.name) {
                Some(Marketplace {
                    platform: "linux-arm64".into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else if is_linux_x86_64(&asset.name) {
                Some(Marketplace {
                    platform: "linux-x64".into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else {
                None
            }
        })
        .unique_by(|a| a.platform.clone())
        .collect_vec();

    x
}

fn is_darwin_aarch64(value: &str) -> bool {
    value.contains("darwin") && value.contains("aarch64")
}

fn is_darwin_x86_64(value: &str) -> bool {
    value.contains("darwin") && value.contains("x86_64")
}

fn is_linux_x86_64(value: &str) -> bool {
    value.contains("linux") && value.contains("x86_64")
}

fn is_linux_aarch64(value: &str) -> bool {
    value.contains("linux") && value.contains("arm")
}
