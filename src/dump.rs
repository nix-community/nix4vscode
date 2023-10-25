use futures::future::join_all;
use log::*;
use semver::Version;

use crate::{
    config::Config,
    data_struct::{self, AssetType},
    jinja::{AssetUrlContext, Generator, NixContext},
    request::{FilterType, HttpClient, ICriterium, IQueryState, Query, RequestFlags},
    utils,
};

pub async fn dump<'a>(
    client: &HttpClient,
    vscode_ver: &Version,
    config: &Config,
    generator: &Generator<'a>,
) -> Vec<NixContext> {
    let make_query = |page_number: u64| Query {
        filters: vec![IQueryState {
            page_number,
            page_size: 9999,
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
        }],
        asset_types: Default::default(),
        flags: RequestFlags::default().bits(),
    };

    let make_request = async move |client: &reqwest::Client,
                                   query: Query|
                -> anyhow::Result<data_struct::IRawGalleryQueryResult> {
        let body = serde_json::to_string(&query)?;
        Ok(client
            .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
            .header(
                "Accept",
                "Application/json; charset=utf-8; api-version=7.2-preview.1",
            )
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?
            .json::<data_struct::IRawGalleryQueryResult>()
            .await?)
    };

    let mut start = 0usize;
    let mut futures = Vec::default();
    loop {
        let query = make_query(start as u64);
        match make_request(&client.client, query).await {
            Ok(res) => {
                if res.results.is_empty() {
                    break;
                }

                futures.extend(
                    res.results
                        .into_iter()
                        .flat_map(|item| item.extensions.into_iter())
                        .map(|item| async move {
                            for version in item.versions {
                                if version.get_engine().matches(vscode_ver) {
                                    continue;
                                }
                                trace!(
                                    "find version {:?} for {}.{}",
                                    version,
                                    item.publisher.publisher_name,
                                    item.extension_name
                                );

                                let (has_asset_url, asset_url) = match config.get_asset_url(
                                    &item.publisher.publisher_name,
                                    &item.extension_name,
                                ) {
                                    Some(url) => {
                                        let url = generator.render_asset_url(
                                            &url,
                                            &AssetUrlContext::new(
                                                config
                                                    .get_system_ctx(
                                                        &item.publisher.publisher_name,
                                                        &item.extension_name,
                                                    )
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
                                        .get_extension_target_platform(
                                            item.publisher.publisher_name,
                                            item.extension_name,
                                        )
                                        .await,
                                });
                            }

                            None
                        }),
                );
            }
            Err(err) => panic!("{err}"),
        }
        start += 1;
    }

    join_all(futures).await.into_iter().flatten().collect()
}
