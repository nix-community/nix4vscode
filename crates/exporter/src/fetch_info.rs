use std::pin::pin;

use crate::models::Marketplace;
use code_api::code::{HttpClient, SortBy};
use diesel::prelude::*;
use futures::StreamExt;

use crate::schema::marketplace;
use tracing::*;

pub async fn fetch_marketplace(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let client = HttpClient::new().unwrap();
    let mut iter = pin!(client.get_extension_response(
        vec![],
        code_api::code::IQueryState {
            page_size: u16::MAX as u64,
            sort_by: SortBy::PUBLISHED_DATE,
            sort_order: code_api::code::SortOrder::DESCENDING,
            ..Default::default()
        }
    ));
    let mut extension_count = 0usize;
    let mut all_count = 0usize;
    while let Some(item) = iter.next().await {
        let Ok(item) = item else {
            continue;
        };

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
                    let visix = v.get_file(code_api::code::AssetType::Vsix)?;
                    let platform = v.target_platform.clone().unwrap_or("universal".to_string());
                    Some(Marketplace {
                        name: item.extension_name.clone(),
                        publisher: item.publisher.publisher_name.clone(),
                        version: v.version.clone(),
                        engine: engne,
                        platform,
                        assert_url: visix.source.clone(),
                        hash: None,
                        is_prerelease: v.is_pre_release_version(),
                    })
                })
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
