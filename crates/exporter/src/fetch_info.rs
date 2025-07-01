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
                    platform: Marketplace::DARWIN_AARCH64.into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else if is_darwin_x86_64(&asset.name) {
                Some(Marketplace {
                    platform: Marketplace::DARWIN_X86.into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else if is_linux_aarch64(&asset.name) {
                Some(Marketplace {
                    platform: Marketplace::LINUX_AARCH64.into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else if is_linux_x86_64(&asset.name) {
                Some(Marketplace {
                    platform: Marketplace::LINUX_X86.into(),
                    url: Some(asset.browser_download_url.to_string()),
                    ..value
                })
            } else {
                None
            }
        })
        .unique_by(|a| a.platform.clone())
        .collect_vec();

    if x.is_empty() {
        return vec![item];
    }

    x
}

fn is_darwin(value: &str) -> bool {
    value.contains("darwin")
}

fn is_linux(value: &str) -> bool {
    value.contains("linux")
}

fn is_aarch64(value: &str) -> bool {
    value.contains("aarch64") || value.contains("arm")
}

fn is_x86_64(value: &str) -> bool {
    value.contains("x86_64") || value.contains("x64")
}

fn is_darwin_aarch64(value: &str) -> bool {
    is_darwin(value) && is_aarch64(value)
}

fn is_darwin_x86_64(value: &str) -> bool {
    is_darwin(value) && is_x86_64(value)
}

fn is_linux_x86_64(value: &str) -> bool {
    is_linux(value) && is_x86_64(value)
}

fn is_linux_aarch64(value: &str) -> bool {
    is_linux(value) && is_aarch64(value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_codelldb() {
        fn nonp(_: &str) -> bool {
            true
        }

        #[allow(clippy::type_complexity)]
        let cases: [(&str, &str, fn(&str) -> bool); 203] = [
            ("v1.9.0", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.9.0", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.9.0", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.9.0", "codelldb-bootstrap.vsix", nonp),
            ("v1.9.0", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.9.0", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.9.0", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.11.5", "codelldb-bootstrap.vsix", nonp),
            ("v1.11.5", "codelldb-darwin-arm64.vsix", is_darwin_aarch64),
            ("v1.11.5", "codelldb-darwin-x64.vsix", is_darwin_x86_64),
            ("v1.11.5", "codelldb-linux-arm64.vsix", is_linux_aarch64),
            ("v1.11.5", "codelldb-linux-armhf.vsix", is_linux_aarch64),
            ("v1.11.5", "codelldb-linux-x64.vsix", is_linux_x86_64),
            ("v1.11.5", "codelldb-win32-x64.vsix", nonp),
            ("v1.7.2", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.7.2", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.7.2", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.7.2", "codelldb-bootstrap.vsix", nonp),
            ("v1.7.2", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.7.2", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.7.2", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.8.1", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.8.1", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.8.1", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.8.1", "codelldb-bootstrap.vsix", nonp),
            ("v1.8.1", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.8.1", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.8.1", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.11.3", "codelldb-bootstrap.vsix", nonp),
            ("v1.11.3", "codelldb-darwin-arm64.vsix", is_darwin_aarch64),
            ("v1.11.3", "codelldb-darwin-x64.vsix", is_darwin_x86_64),
            ("v1.11.3", "codelldb-linux-arm64.vsix", is_linux_aarch64),
            ("v1.11.3", "codelldb-linux-armhf.vsix", is_linux_aarch64),
            ("v1.11.3", "codelldb-linux-x64.vsix", is_linux_x86_64),
            ("v1.11.3", "codelldb-win32-x64.vsix", nonp),
            ("v1.11.0", "codelldb-bootstrap.vsix", nonp),
            ("v1.11.0", "codelldb-darwin-arm64.vsix", is_darwin_aarch64),
            ("v1.11.0", "codelldb-darwin-x64.vsix", is_darwin_x86_64),
            ("v1.11.0", "codelldb-linux-arm64.vsix", is_linux_aarch64),
            ("v1.11.0", "codelldb-linux-armhf.vsix", is_linux_aarch64),
            ("v1.11.0", "codelldb-linux-x64.vsix", is_linux_x86_64),
            ("v1.11.0", "codelldb-win32-x64.vsix", nonp),
            ("v1.7.3", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.7.3", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.7.3", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.7.3", "codelldb-bootstrap.vsix", nonp),
            ("v1.7.3", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.7.3", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.7.3", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.10.0", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.10.0", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.10.0", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.10.0", "codelldb-bootstrap.vsix", nonp),
            ("v1.10.0", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.10.0", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.10.0", "codelldb-x86_64-windows.vsix", nonp),
            (
                "v1.6.8-dev.2110042330",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.6.8-dev.2110042330",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.6.8-dev.2110042330",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.6.8-dev.2110042330", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.6.8-dev.2110042330",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.6.8-dev.2110042330",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.6.8-dev.2110042330",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            ("v1.9.1", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.9.1", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.9.1", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.9.1", "codelldb-bootstrap.vsix", nonp),
            ("v1.9.1", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.9.1", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.9.1", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.6.6", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.6.6", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.6.6", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.6.6", "codelldb-bootstrap.vsix", nonp),
            ("v1.6.6", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.6.6", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.6.6", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.8.0", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.8.0", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.8.0", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.8.0", "codelldb-bootstrap.vsix", nonp),
            ("v1.8.0", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.8.0", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.8.0", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.9.2", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.9.2", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.9.2", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.9.2", "codelldb-bootstrap.vsix", nonp),
            ("v1.9.2", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.9.2", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.9.2", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.7.4", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.7.4", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.7.4", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.7.4", "codelldb-bootstrap.vsix", nonp),
            ("v1.7.4", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.7.4", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.7.4", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.6.8", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.6.8", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.6.8", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.6.8", "codelldb-bootstrap.vsix", nonp),
            ("v1.6.8", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.6.8", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.6.8", "codelldb-x86_64-windows.vsix", nonp),
            (
                "v1.9.0-dev.2302150256",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.9.0-dev.2302150256",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.9.0-dev.2302150256",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.9.0-dev.2302150256", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.9.0-dev.2302150256",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.9.0-dev.2302150256",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.9.0-dev.2302150256",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            (
                "v1.6.7-dev.2109102249",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.6.7-dev.2109102249",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.6.7-dev.2109102249",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.6.7-dev.2109102249", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.6.7-dev.2109102249",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.6.7-dev.2109102249",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.6.7-dev.2109102249",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            ("v1.6.9", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.6.9", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.6.9", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.6.9", "codelldb-bootstrap.vsix", nonp),
            ("v1.6.9", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.6.9", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.6.9", "codelldb-x86_64-windows.vsix", nonp),
            (
                "v1.6.8-dev.2110040647",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.6.8-dev.2110040647",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.6.8-dev.2110040647",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.6.8-dev.2110040647", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.6.8-dev.2110040647",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.6.8-dev.2110040647",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.6.8-dev.2110040647",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            ("v1.11.4", "codelldb-bootstrap.vsix", nonp),
            ("v1.11.4", "codelldb-darwin-arm64.vsix", is_darwin_aarch64),
            ("v1.11.4", "codelldb-darwin-x64.vsix", is_darwin_x86_64),
            ("v1.11.4", "codelldb-linux-arm64.vsix", is_linux_aarch64),
            ("v1.11.4", "codelldb-linux-armhf.vsix", is_linux_aarch64),
            ("v1.11.4", "codelldb-linux-x64.vsix", is_linux_x86_64),
            ("v1.11.4", "codelldb-win32-x64.vsix", nonp),
            ("v1.11.2", "codelldb-bootstrap.vsix", nonp),
            ("v1.11.2", "codelldb-darwin-arm64.vsix", is_darwin_aarch64),
            ("v1.11.2", "codelldb-darwin-x64.vsix", is_darwin_x86_64),
            ("v1.11.2", "codelldb-linux-arm64.vsix", is_linux_aarch64),
            ("v1.11.2", "codelldb-linux-armhf.vsix", is_linux_aarch64),
            ("v1.11.2", "codelldb-linux-x64.vsix", is_linux_x86_64),
            ("v1.11.2", "codelldb-win32-x64.vsix", nonp),
            (
                "v1.7.0-dev.2203050810",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.7.0-dev.2203050810",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.7.0-dev.2203050810",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.7.0-dev.2203050810", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.7.0-dev.2203050810",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.7.0-dev.2203050810",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.7.0-dev.2203050810",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            ("v1.11.1", "codelldb-bootstrap.vsix", nonp),
            ("v1.11.1", "codelldb-darwin-arm64.vsix", is_darwin_aarch64),
            ("v1.11.1", "codelldb-darwin-x64.vsix", is_darwin_x86_64),
            ("v1.11.1", "codelldb-linux-arm64.vsix", is_linux_aarch64),
            ("v1.11.1", "codelldb-linux-armhf.vsix", is_linux_aarch64),
            ("v1.11.1", "codelldb-linux-x64.vsix", is_linux_x86_64),
            ("v1.11.1", "codelldb-win32-x64.vsix", nonp),
            ("v1.7.1", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.7.1", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.7.1", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.7.1", "codelldb-bootstrap.vsix", nonp),
            ("v1.7.1", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.7.1", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.7.1", "codelldb-x86_64-windows.vsix", nonp),
            (
                "v1.7.2-dev.2207161821",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.7.2-dev.2207161821",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.7.2-dev.2207161821",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.7.2-dev.2207161821", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.7.2-dev.2207161821",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.7.2-dev.2207161821",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.7.2-dev.2207161821",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            (
                "v1.6.7-dev.2109180641",
                "codelldb-aarch64-darwin.vsix",
                is_darwin_aarch64,
            ),
            (
                "v1.6.7-dev.2109180641",
                "codelldb-aarch64-linux.vsix",
                is_linux_aarch64,
            ),
            (
                "v1.6.7-dev.2109180641",
                "codelldb-arm-linux.vsix",
                is_linux_aarch64,
            ),
            ("v1.6.7-dev.2109180641", "codelldb-bootstrap.vsix", nonp),
            (
                "v1.6.7-dev.2109180641",
                "codelldb-x86_64-darwin.vsix",
                is_darwin_x86_64,
            ),
            (
                "v1.6.7-dev.2109180641",
                "codelldb-x86_64-linux.vsix",
                is_linux_x86_64,
            ),
            (
                "v1.6.7-dev.2109180641",
                "codelldb-x86_64-windows.vsix",
                nonp,
            ),
            ("v1.6.10", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.6.10", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.6.10", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.6.10", "codelldb-bootstrap.vsix", nonp),
            ("v1.6.10", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.6.10", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.6.10", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.6.7", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.6.7", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.6.7", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.6.7", "codelldb-bootstrap.vsix", nonp),
            ("v1.6.7", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.6.7", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.6.7", "codelldb-x86_64-windows.vsix", nonp),
            ("v1.7.0", "codelldb-aarch64-darwin.vsix", is_darwin_aarch64),
            ("v1.7.0", "codelldb-aarch64-linux.vsix", is_linux_aarch64),
            ("v1.7.0", "codelldb-arm-linux.vsix", is_linux_aarch64),
            ("v1.7.0", "codelldb-bootstrap.vsix", nonp),
            ("v1.7.0", "codelldb-x86_64-darwin.vsix", is_darwin_x86_64),
            ("v1.7.0", "codelldb-x86_64-linux.vsix", is_linux_x86_64),
            ("v1.7.0", "codelldb-x86_64-windows.vsix", nonp),
        ];

        for (version, filename, predicate) in cases {
            assert!(predicate(filename), "{version} - {filename} assert failed");
        }
    }
}
